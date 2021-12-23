#!/usr/bin/python3

import collections
import fileinput
import functools
import heapq
import itertools
import math
import re
import sys

MOVE_COSTS={ "A": 1, "B": 10, "C": 100, "D": 1000 }

HALL_LEN=11

ROOMS_AT=[ (2,"A"), (4, "B"), (6, "C"), (8, "D") ]
ROOMS=len(ROOMS_AT)
NO_STOP=set([ pos for (pos, pod) in ROOMS_AT ])
WANT_ROOM={ "A": 0, "B": 1, "C": 2, "D": 3 }

def solve(inp):
    rooms = [ [c for c in l.strip()] for l in inp ]
    print(rooms)

    start = State(hall=[None]*11, rooms=rooms)
    visited = set()
    heap = [(0, start, ())]
    solutions = []
    while heap:
        cost, state, path = heapq.heappop(heap)
        if state in visited:
            print("already visited", state)
            continue

        print("cost", cost, "state", state)
        visited.add(state)
        
        if state.final():
            print("final", cost)
            for st in path:
                print(st)
            return

        for move_cost, move_to_state in state.valid_moves():
            heapq.heappush(heap, (cost+move_cost, move_to_state, tuple(list(path) + [state])) )


class State(object):
    def __init__(self, hall, rooms):
        assert(len(hall) == HALL_LEN)
        assert(len(rooms) == ROOMS)
        assert(all([ len(room) == 2 for room in rooms ]))

        self._hall = tuple(hall)
        self._rooms = tuple([tuple(room) for room in rooms])

    def __repr__(self):
        return f"State(hall={self._hall}, rooms={self._rooms})"

    def __hash__(self):
        return hash((self._hall,self._rooms))

    def __eq__(self, other):
        return (self._hall,self._rooms) == (other._hall,other._rooms)

    def __lt__(self, other):
        return False
    
    def final(self):
        return all([room[0] == want and room[1] == want for (room, (ignored, want)) in zip(self._rooms, ROOMS_AT)])

    # Returns [ (cost, State), (cost, State), ... ] in ascending cost order.
    def valid_moves(self):
        # * Amphipods will never stop on the space immediately outside any room.
        # * Amphipods will never move from the hallway into a room unless that room is their destination room
        #   and that room contains no amphipods which do not also have that room as their own destination.
        # * Once an amphipod stops moving in the hallway, it will stay in that spot until it can move into a room.

        out = []

        # Compute moves for pods in the hall
        for i in range(HALL_LEN):
            out.extend(self._moves_from_hall(i))

        for i in range(ROOMS):
            out.extend(self._moves_from_room(i))

        return sorted(out)

    # Assuming a pod is standing right outside their wanted room, what
    # valid moves are there?
    #
    # Returns None or (cost, new rooms array).
    def _moves_into_wanted_room(self, pod):
        room_idx = WANT_ROOM[pod]
        room = self._rooms[room_idx]
        steps = 2
        if room[0]:
            # Room is full.
            return None
        if room[1]:
            # Back of room has a pod.
            if room[1] != pod:
                # But not `pod`.
                return None
            steps = 1

        # Create a new room array usable for State()
        rooms = [list(room[:]) for room in self._rooms]
        rooms[room_idx][steps-1] = pod
        return (MOVE_COSTS[pod] * steps, tuple([tuple(room) for room in rooms]))

    # Returns any valid states starting from idx in hall, with costs
    def _moves_from_hall(self, idx):
        if not self._hall[idx]:
            # no pod here
            return []
        pod = self._hall[idx]
        pod_room_idx = ROOMS_AT[WANT_ROOM[pod]][0]
        for check_idx in between(idx, pod_room_idx):
            # Is there a pod in the way?
            if self._hall[check_idx]:
                return []
        # No pods in the way, check if we can get in.
        room_move = self._moves_into_wanted_room(pod)
        if not room_move:
            return []

        # We can move to pod to its room.
        cost, new_rooms = room_move
        cost += dist(idx, pod_room_idx) * MOVE_COSTS[pod]
        hall = list(self._hall[:])
        hall[idx] = None
        hall = tuple(hall)
        return [ (cost, State(hall=hall, rooms=new_rooms)) ]

    def _moves_from_room_to_hall(self, room_idx):
        room_pos, pod_wanted = ROOMS_AT[room_idx]
        room = self._rooms[room_idx]

        if not room[0] and not room[1]:
            # nothing here
            return None

        if room[1] == pod_wanted:
            # back of room already set, only consider front
            if not room[0] or room[0] == pod_wanted:
                # either already what we want, or empty
                return None
            # the back of the room is set but there's a pod in the front
            steps = 1
        elif not room[0]:
            # there's a pod in the back of the room
            steps = 2
        else:
            steps = 1

        rooms = [list(room[:]) for room in self._rooms]
        pod = rooms[room_idx][steps-1]
        rooms[room_idx][steps-1] = None
        return (MOVE_COSTS[pod] * steps, pod, room_pos, tuple([tuple(room) for room in rooms]))

    def _moves_from_room(self, room_idx):
        found = self._moves_from_room_to_hall(room_idx)
        if not found:
            return []

        cost, pod, room_pos, new_rooms = found
        # A pod can leave the room and is now at position `pos`.

        possible_stops = []
        for pos in between(room_pos, HALL_LEN):
            if self._hall[pos]:
                # There's a pod in the way.
                break
            possible_stops.append(pos)
        for pos in between(room_pos, -1):
            if self._hall[pos]:
                # There's a pod in the way.
                break
            possible_stops.append(pos)

        out = []
        for stop in possible_stops:
            if stop in NO_STOP:
                continue
            new_hall = list(self._hall[:])
            new_hall[stop] = pod
            new_hall = tuple(new_hall)
            out.append( (cost + MOVE_COSTS[pod] * dist(room_pos, stop), State(hall=new_hall, rooms=new_rooms) ) )

        return out

def between(idx1, idx2):
    if idx1 < idx2:
        return range(idx1+1, idx2)
    return range(idx1-1, idx2, -1)

def dist(idx1, idx2):
    return abs(idx1-idx2)

def tests():
    assert State(hall = [None]*11, rooms=[["A"]*2, ["B"]*2, ["C"]*2, ["D"]*2]).final()
    assert not State(hall = [None]*11, rooms=[["A",'B'], ["B",'A'], ["C"]*2, ["D"]*2]).final()
    print(State(hall = ['A'] + [None]*10, rooms=[[None,'B'], ["B",'A'], ["C"]*2, ["D"]*2])._moves_into_wanted_room('A'))
    print(State(hall = ['A'] + [None]*10, rooms=[[None,'A'], ["B",'B'], ["C"]*2, ["D"]*2])._moves_into_wanted_room('A'))
    print(State(hall = ['A','A'] + [None]*9, rooms=[[None,None], ["B",'B'], ["C"]*2, ["D"]*2])._moves_into_wanted_room('A'))
    print(list(between(5,9)))
    print(list(between(9,5)))
    print(dist(9,5))
    print(State(hall = ['B'] + [None]*10, rooms=[['A']*2, [None,'B'], ["C"]*2, ["D"]*2])._moves_from_hall(0))
    print(State(hall = ['A','B'] + [None]*9, rooms=[[None,'A'], [None,None], ["C"]*2, ["D"]*2])._moves_from_hall(1))
    print(State(hall = ['B','A'] + [None]*9, rooms=[[None,'A'], [None,'B'], ["C"]*2, ["D"]*2])._moves_from_hall(0))
    print(State(hall = [None]*11, rooms=[[None,'A'], [None,'B'], ["C"]*2, ["D"]*2])._moves_from_room_to_hall(1))
    print(State(hall = [None]*11, rooms=[[None,'B'], [None,'A'], ["C"]*2, ["D"]*2])._moves_from_room_to_hall(1))
    print(State(hall = [None]*11, rooms=[[None,'B'], ['B','A'], ["C"]*2, ["D"]*2])._moves_from_room_to_hall(1))
    print(State(hall = [None, 'C'] + [None]*9, rooms=[['B']*2, ['A']*2, ["C"]*2, ["D"]*2])._moves_from_room(1))
    print(State(hall = [None]*11, rooms=[['B']*2, ['A']*2, ["C"]*2, ["D"]*2]).valid_moves())

    # Verify hashable
    st1 = State(hall = [None]*11, rooms=[['B']*2, ['A']*2, ["C"]*2, ["D"]*2])
    st2 = State(hall = [None]*11, rooms=[['B']*2, ['A']*2, ["C"]*2, ["D"]*2])
    s = {st1, st2}
    assert(len(s) == 1)

    print(State(hall=(None, None, None, 'B', None, None, None, None, None, None, None), rooms=(('B', 'A'), ('C', 'D'), (None, 'C'), ('D', 'A'))).valid_moves())
    print(State(hall=(None, None, None, 'B', None, 'C', None, None, None, None, None), rooms=(('B', 'A'), (None, 'D'), (None, 'C'), ('D', 'A'))).valid_moves())
    print(State(hall=(None, None, None, None, None, 'D', None, 'D', None, None, None), rooms=((None, 'A'), ('B', 'B'), ('C', 'C'), (None, 'A'))).valid_moves())
    print("--- tests done ---")

if __name__ == "__main__":
    tests()
    solve(fileinput.input(sys.argv[1]))
