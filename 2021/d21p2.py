#!/usr/bin/python3

import collections
import fileinput
import functools
import itertools
import math
import re
import sys

NUM_PLAYERS = 2
WIN_SCORE = 21

def solve(inp):
    pos = [int(l.strip().split(":")[1]) for l in inp if l.strip() != ""]
    assert(len(pos) == NUM_PLAYERS)

    roll_states = collections.defaultdict(int)
    for r1 in range(1,4):
        for r2 in range(1,4):
            for r3 in range(1,4):
                roll_states[r1+r2+r3] += 1
    
    states = collections.defaultdict(int)
    states[ ((pos[0], 0), (pos[1], 0)) ] = 1
    win_universes = [ 0 for i in range(NUM_PLAYERS) ]
    player = 0
    while states:
        new_states = collections.defaultdict(int)
        for state, universes in states.items():
            pos, score = state[player]
            for roll, roll_universes in roll_states.items():
                new_pos = ((pos+roll-1) % 10)+1
                new_score = score + new_pos
                new_universes = universes * roll_universes
                if new_score >= WIN_SCORE:
                    win_universes[player] += new_universes
                else:
                    new_state = list(state)
                    new_state[player] = (new_pos, new_score)
                    new_state = tuple(new_state)
                    new_states[new_state] += new_universes
        states = new_states
        player = (player + 1) % NUM_PLAYERS
        print(len(states))
    print(win_universes)
    print(max(win_universes))
    
if __name__ == "__main__":
    solve(fileinput.input(sys.argv[1]))
