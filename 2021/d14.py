#!/usr/bin/python3

import collections
import fileinput
import itertools
import math
import re
import sys

def solve(inp):
    start = next(inp).strip()
    next(inp)
    rules = {}
    for l in inp:
        pair, middle = l.strip().split()
        rules[ (pair[0], pair[1]) ] =  middle 
    print(rules)
    
if __name__ == "__main__":
    solve(fileinput.input(sys.argv[1]))
