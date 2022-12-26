#!/usr/bin/python3

import collections
import fileinput
import functools
import heapq
import itertools
import math
import multiprocessing
import re
import sys
import json

TURN_MAP = {
    'R': {
        (1,0): (0,1),
        (0,1): (-1,0),
        (-1,0): (0,-1),
        (0,-1): (1,0)
    },
    'L': {
        (1,0): (0,-1),
        (0,-1): (-1,0),
        (-1,0): (0,1),
        (0,1): (1,0)
    }
}

FACING_VALUE = {
        (1,0): 0,
        (0,1): 1,
        (-1,0): 2,
        (0,-1): 3,
}

# I probably should have made these the same order as FACING_VALUE but I don't want to fix it.
# Anti-clockwise from right.
CUBE_FACE_ORDER = (
    (1,0),
    (0,-1),
    (-1,0),
    (0,1)
)

# each face and its right, up, left, down border
CUBE_FACES = {
    'yellow': ['green', 'orange', 'blue', 'red'],
    'green': ['white', 'orange', 'yellow', 'red'],
    'orange': ['yellow', 'green', 'white', 'blue'],
    'red': ['yellow', 'blue', 'white', 'green'],
    'blue': ['yellow', 'orange', 'white', 'red'],
    'white': ['green', 'red', 'blue', 'orange'],
}

grid = {}
x_max = 0
y_max = 0

def solve(inp):
    global grid
    global x_max
    global y_max
    global face_size
    global face_assignment
    global face_assignment_to_coords
    global face_facing

    y = 0
    beginning = None
    for l in inp:
        l = l.rstrip()
        if not l:
            break

        for x in range(len(l)):
            if l[x] == '.':
                grid[(x,y)] = False
                if not beginning:
                    beginning = (x,y)
            elif l[x] == '#':
                grid[(x,y)] = True
            # skip spaces

        x_max = max(x_max, len(l))
        y += 1
    y_max = y

    directions = next(inp).strip()
    assert(directions)
    directions = re.split(r'([RL])', directions)

    face_size = math.gcd(x_max, y_max) # seems goofy but works
    face_coords = [] # by top left coord
    for y in range(0, y_max, face_size):
        for x in range(0, x_max, face_size):
            if (x,y) in grid:
                assert (x+face_size-1,y+face_size-1) in grid
                face_coords.append((x,y))
    print(face_coords)
    assert len(face_coords) == 6

    for face_order in itertools.permutations(CUBE_FACES.keys(), 6):
        face_assignment = dict(zip(face_coords, face_order))
        face_facing = viable_face_assignment(face_assignment, face_size)
        if face_facing:
            break

    face_assignment_to_coords = { face:coords for (coords, face) in face_assignment.items() }
    print(face_assignment)
    print(face_facing)

    # print(grid)
    # print(directions)

    assert beginning

    loc = beginning
    facing = (1,0)

    for direction in directions:
        print(f"loc: {loc}, facing: {facing}, direction: {direction}")
        if direction.isnumeric():
            for _ in range(int(direction)):
                new_loc, new_facing = step(loc, facing)
                if grid[new_loc]:
                    print(f"  rock at {new_loc}, movement stopped")
                    break
                loc = new_loc
                facing = new_facing
                print(f"  step to loc: {loc}")
        else:
            facing = TURN_MAP[direction][facing]
        
        print(f"after '{direction}' loc: {loc}, facing: {facing}")

    print((loc[0]+1)*4 + (loc[1]+1)*1000 + FACING_VALUE[facing])

def viable_face_assignment(face_assignment, face_size):
    face_facing = {}
    for (x,y), face in face_assignment.items():
        face_matches = None
        for i in range(4): # neighbor face order
            neighbor_faces = CUBE_FACES[face][i:] + CUBE_FACES[face][:i]
            matches = True
            for j, (dx, dy) in zip(range(4), [(face_size, 0), (0, -face_size), (-face_size, 0), (0, face_size)]):
                assigned_face = face_assignment.get((x+dx, y+dy))
                if not assigned_face:
                    continue
                if assigned_face != neighbor_faces[j]:
                    matches = False
            if matches:
                face_matches = neighbor_faces
                break
        if not face_matches:
            return False
        face_facing[face] = neighbor_faces
    return face_facing

@functools.cache
def step(loc, facing):
    global grid
    global face_size
    global face_assignment
    global face_assignment_to_coords
    global face_facing

    new_loc = (loc[0]+facing[0], loc[1]+facing[1])
    if new_loc in grid:
        return new_loc, facing

    # find what face loc is in
    for (x_face,y_face), face in face_assignment.items():
        if x_face <= loc[0] < x_face+face_size and y_face <= loc[1] < y_face+face_size:
            break

    # position in face
    x_in_face, y_in_face = loc[0] - x_face, loc[1] - y_face

    dir_idx = CUBE_FACE_ORDER.index(facing) # 0 = right, anticlockwise to 3 == down
    other_face = face_facing[face][dir_idx] # next face after edge
    other_dir_idx = face_facing[other_face].index(face) # direction of face from other face
    other_face_loc = face_assignment_to_coords[other_face]

    # face orientation, 2 == faces are oriented the same way, 1 means face needs to turn 90 clockwise to meet other on the grid, 3 means 90 anticlockwise
    orientation = (other_dir_idx - dir_idx) % 4
    new_facing = CUBE_FACE_ORDER[(other_dir_idx + 2) % 4] # opposite direction from the face it's coming from
    print(f"    loc {loc} is on face {face}; in_face {x_in_face, y_in_face}; dir_idx {dir_idx}; other face {other_face} at {other_face_loc}; other_dir_idx {other_dir_idx}; orientation {orientation}; new_facing {new_facing}")

    if orientation == 2:
        # basically just keep going, but as if teleported to the other face
        assert new_facing == facing
        return (other_face_loc[0] + (x_in_face+facing[0]) % face_size, other_face_loc[1] + (y_in_face+facing[1]) % face_size), new_facing

    '''
    orientation == 1:
    0123 => 3210
    1          1
    2          2
    3          3

    so new y = old x, and new x = face_size - 1 - old y

    orientation == 3:
    0123 => 3
    1       2
    2       1
    3       0123

    so new y = face_size - 1 - old x, x = old y

    BUT, we only need to care about the "non-directional" axis as the other can be inferred from the border coords (0 or face_size-1)
    '''
    if new_facing[0]:
        border_x = other_face_loc[0] + (0 if new_facing[0] > 0 else face_size - 1)
        if orientation == 0:
            return (border_x, other_face_loc[1]+face_size-1-y_in_face), new_facing
        elif orientation == 1:
            return (border_x, other_face_loc[1]+x_in_face), new_facing
        else:
            assert orientation == 3
            return (border_x, other_face_loc[1]+face_size-1-x_in_face), new_facing
    else:
        assert new_facing[1]
        border_y = other_face_loc[1] + (0 if new_facing[1] > 0 else face_size - 1)
        if orientation == 0:
            return (other_face_loc[0]+face_size-1-x_in_face, border_y), new_facing
        elif orientation == 1:
            return (other_face_loc[0]+face_size-1-y_in_face, border_y), new_facing
        else:
            assert orientation == 3
            return (other_face_loc[0]+y_in_face, border_y), new_facing

    assert False, "shouldn't get here"

def tests():
    print("--- tests done ----")

if __name__ == "__main__":
    tests()
    solve(fileinput.input(sys.argv[1]))
