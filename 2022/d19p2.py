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

TYPES={
    'ore': 0,
    'clay': 1,
    'obsidian': 2,
    'geode': 3,
}
def solve(inp):
    blueprints = {}
    for l in inp:
        name, cost_strings = l.strip().split(": ")
        num = int(name.split(" ")[1])
        cost_strings = cost_strings.strip(".").split(". ")
        costs = [None] * 4
        for cost_string in cost_strings:
            what_string, robot_cost_strings = cost_string.split(" robot costs ")
            what = TYPES[what_string.split(" ")[1]]
            robot_costs = [0] * 4
            for robot_cost_string in robot_cost_strings.split(" and "):
                cnt, thing = robot_cost_string.split(" ")
                robot_costs[TYPES[thing]] = int(cnt)
                
            costs[what] = tuple(robot_costs)
        blueprints[num] = tuple(costs)
        if len(blueprints) == 1:
            break

    # print(blueprints)

    with multiprocessing.Pool(processes=len(blueprints)) as pool:
        geodes = pool.map(geodes_for_blueprint, blueprints.items())
        print(geodes)
        print(geodes[0]*geodes[1]*geodes[2])

def geodes_for_blueprint(args):
    num, blueprint = args
    global best_geodes
    best_geodes = 30
    geodes = blueprint_geodes(blueprint)
    print(num, geodes)
    return geodes

best_geodes=30

def blueprint_geodes(costs, robots=(1,0,0,0), have=(0,0,0,0), time_left=32):
    global best_geodes

    have_geodes = have[TYPES["geode"]]
    if best_geodes > have_geodes + sum(range(robots[TYPES["geode"]], robots[TYPES["geode"]]+time_left+1)):
        return -1
    if time_left == 0:
        if have_geodes > best_geodes:
            print("  ", best_geodes, have_geodes, robots, have)
            best_geodes = have_geodes
        return have_geodes

    available_moves = [None] # None == do nothing, always available
    for maybe_buy in range(4):
        if all([have[i] >= costs[maybe_buy][i] for i in range(4)]):
            available_moves.append(maybe_buy)

    next_have = tuple([ have[i] + robots[i] for i in range(4) ])

    best = 0
    for move in available_moves: # reversed => greedy
        if move == None:
            result = blueprint_geodes(costs, robots, next_have, time_left-1)
        else:
            next_after_paying = tuple([ next_have[i] - costs[move][i] for i in range(4) ])
            next_robots = tuple([ robots[i] + (1 if i == move else 0) for i in range(4) ])
            result = blueprint_geodes(costs, next_robots, next_after_paying, time_left-1)
        best = max(best, result)

    return best        


def tests():
    print("--- tests done ----")

if __name__ == "__main__":
    tests()
    solve(fileinput.input(sys.argv[1]))
