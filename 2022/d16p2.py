#!/usr/bin/python3

import collections
import fileinput
import functools
import heapq
import itertools
import math
import re
import sys
import json

distances = collections.defaultdict(dict)
income = {}
useful_valves = set()

def solve(inp):
    for l in inp:
        # Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        m = re.match("Valve ([^ ]+) has flow rate=([0-9]+); tunnels? leads? to valves? (.*)", l)
        assert m
        valve, rate, edges = m.groups()
        income[valve] = int(rate)

        distances[valve][valve] = 0
        for next_valve in edges.split(', '):
            distances[valve][next_valve] = 1

    for valve, valve_to in itertools.product(distances.keys(), distances.keys()):
        distances[valve][valve_to] = distances[valve].get(valve_to, 10000000000)

    changed = True
    while changed:
        changed = False
        for valve, distances_to_next_valve in distances.items():
            for next_valve, distance_from_valve_to_next_valve in distances_to_next_valve.items():
                for next_next_valve, distance_from_next_valve_to_next_next_valve in distances[next_valve].items():
                    maybe = distance_from_valve_to_next_valve + distance_from_next_valve_to_next_next_valve
                    if maybe < distances[valve][next_next_valve]:
                        distances[valve][next_next_valve] = maybe
                        changed = True

    # distances is now the time cost to get from a -> b
    # for valve, distances_to_next_valve in distances.items():
    #     print(valve, distances_to_next_valve)

    for valve, valve_income in income.items():
        if valve_income > 0:
            useful_valves.add(valve)
    # print(useful_valves)

    best = find_best(['AA', 'AA'], set(), 0, [26, 26])
    print(best)

def find_best(at_valve, enabled, profit, time_left):
    # time_left is list of amount of time left, [me, elephant].
    # we fast-forward to the next decision points ater making
    # a move.
    #
    # at_valve is list of positions, [me, elephant]. However,
    # the position isn't reached until time_left is maximum
    # for that entity.
    t_minus = max(time_left)
    whose_move = [ i for i in range(len(time_left)) if time_left[i] == t_minus ]

    if t_minus > 20:
        print(at_valve, enabled, profit, time_left, whose_move)

    best = profit
    for valve in useful_valves - enabled:
        for who in whose_move:
            d = distances[at_valve[who]][valve]
            time_after_enable = t_minus - d - 1
            if time_after_enable <= 0:
                continue # no time left to make profit
            maybe_profit = time_after_enable * income[valve]

            new_at_valve = at_valve[:]
            new_at_valve[who] = valve

            new_time_left = time_left[:]
            new_time_left[who] = time_after_enable

            best_for_valve = find_best(new_at_valve, enabled.union({valve}), profit+maybe_profit, new_time_left)
            best = max(best, best_for_valve)
    return best

def tests():
    print("--- tests done ----")

if __name__ == "__main__":
    tests()
    solve(fileinput.input(sys.argv[1]))
