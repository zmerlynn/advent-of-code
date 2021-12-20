#!/usr/bin/python3

import collections
import fileinput
import functools
import itertools
import math
import re
import sys

def solve(inp):
    scanners = []
    while True:
        try:
            header = next(inp)
        except StopIteration:
            break

        coords = []
        while True:
            try:
                l = next(inp).strip()
            except StopIteration:
                break
            if l == "":
                break
            coords.append(tuple([ int(s) for s in l.split(",") ]))

        scanners.append(coords)

    coord_pair_by_distance = collections.defaultdict(list)
    distance_sets_by_scanner = [ set() for i in range(len(scanners)) ]
    for sc in range(len(scanners)):
        coords = scanners[sc]
        for x in range(len(coords)):
            for y in range(x):
                d = dsq(coords[x], coords[y])
                coord_pair_by_distance[d].append( (sc, x, y) )
                distance_sets_by_scanner[sc].add(d)

    scanner_overlaps = []
    most_common_pair = None
    most_common_cardinality = 0
    distances_in_common = None
    for sc2 in range(len(scanners)):
        for sc1 in range(sc2):
            isect = distance_sets_by_scanner[sc1].intersection(distance_sets_by_scanner[sc2])
            if len(isect) == 66:
                scanner_overlaps.append( (sc1, sc2, isect) )

    # sc1 -> [ (sc2, [(sc1pt, sc2pt), ...] ) ]
    overlaps = collections.defaultdict(list)
    for sc1, sc2, common_distances in scanner_overlaps:
        common_points = compute_common_points(coord_pair_by_distance, sc1, sc2, common_distances)
        overlaps[sc1].append( (sc2, common_points) )
        overlaps[sc2].append( (sc1, [ (sc2pt, sc1pt) for (sc1pt, sc2pt) in common_points ] ) )
    print(overlaps)

    assert(0 in overlaps)
    normalized_scanners = {0}
    scanner_positions = []
    while len(normalized_scanners) != len(scanners):
        new_normal = set(normalized_scanners)
        for sc1 in normalized_scanners:
            for (sc2, common_points) in overlaps[sc1]:
                if sc2 not in new_normal:
                    scanner_positions.append(normalize(scanners, sc1, sc2, common_points))
                    new_normal.add(sc2)
        normalized_scanners = new_normal

    beacons = set()
    for coords in scanners:
        for coord in coords:
            beacons.add(coord)
    print(len(beacons))

    max_d = 0
    for p1 in range(len(scanner_positions)):
        for p2 in range(p1):
            d = sum([abs(scanner_positions[p1][i]-scanner_positions[p2][i]) for i in range(3)])
            max_d = max(max_d, d)
    print(max_d)

# normal and relative are both indices within scanners[]
def normalize(scanners, normal, relative, common_points):
    print("using scanner", normal, "to normalize scanner", relative, "common points:", common_points)
    common_normal = []
    common_relative = []
    for (sc1pt, sc2pt) in common_points:
        common_normal.append(scanners[normal][sc1pt])
        common_relative.append(scanners[relative][sc2pt])

    orientation = None
    for possible_orientation in orientations():
        common_relative_reoriented = reorient(common_relative, possible_orientation)
        if prove_orientation(common_normal, common_relative_reoriented):
            common_relative = common_relative_reoriented
            orientation = possible_orientation
            break
    if not orientation:
        raise Exception("could not orient")
    # print("orientation", orientation)

    # print(common_normal)
    # print(common_relative)
    scanner_pos = tuple([ normal-relative for (relative,normal) in zip(common_relative[0], common_normal[0]) ])
    # print("scanner pos", scanner_pos)

    coords = reorient(scanners[relative], orientation)
    for coord_idx in range(len(coords)):
        # print("prenormal", coords[coord_idx])
        coords[coord_idx] = tuple([ coords[coord_idx][i]+scanner_pos[i] for i in range(3) ])
        # print("normal", coords[coord_idx])
    scanners[relative] = coords
    return scanner_pos
        
def orientations():
    for order in itertools.permutations([0,1,2]):
        for x_sign in [1, -1]:
            for y_sign in [1, -1]:
                for z_sign in [1, -1]:
                    yield ( (x_sign,order[0]), (y_sign,order[1]), (z_sign,order[2]) )

def prove_orientation(normal, possible):
    for p1 in range(len(normal)):
        for p2 in range(p1):
            for i in range(3):
                if normal[p1][i]-normal[p2][i] != possible[p1][i]-possible[p2][i]:
                    return False
    return True

def reorient(coords, ori):
    out = []
    for coord in coords:
        out.append(tuple([ sign*coord[i] for (sign,i) in ori ]))
    return out
    
# returns common points in (sc1, sc2) order (by index into scanner)
def compute_common_points(coord_pair_by_distance, sc1, sc2, common_distances):
    # for a given scanner pair (sc1, sc2)..
    # coord index in sc1 -> set of possible coords in sc2
    possible_points_lr = {}
    for d in common_distances:
        in_pair = [ (sc, x, y) for (sc, x, y) in coord_pair_by_distance[d] if sc in [sc1, sc2] ]
        if len(in_pair) != 2:
            continue
        infer(possible_points_lr, in_pair[0][1], in_pair[1][1], in_pair[1][2])
        infer(possible_points_lr, in_pair[0][2], in_pair[1][1], in_pair[1][2])
    if len(possible_points_lr) != 12:
        raise Exception("confused? not 12 possible points")
    return [ (it[0], list(it[1])[0]) for it in possible_points_lr.items() ]

def infer(possible_points_map, point, maybe1, maybe2):
    maybes = {maybe1, maybe2}
    if point not in possible_points_map:
        possible_points_map[point] = maybes
    else:
        possible_points_map[point] = possible_points_map[point].intersection( maybes )

def dsq(x, y):
    return (x[0]-y[0])**2 + (x[1]-y[1])**2 + (x[2]-y[2])**2

if __name__ == "__main__":
    solve(fileinput.input(sys.argv[1]))
