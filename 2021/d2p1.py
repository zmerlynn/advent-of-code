#!/usr/bin/python3

import numpy as np
import fileinput
import sys

# facing [1, 0, 0], up is [0, 0, 1]
command_map = {
    "forward": np.array([1, 0, 0]),
    "down": np.array([0, 0, -1]),
    "up": np.array([0, 0, 1])
}

def solve(inp):
    pos = np.array([0, 0, 0])
    for line in inp:
        cmd, arg = line.split()
        delta = command_map[cmd] * int(arg)
        pos += delta
        print(f'{cmd} {arg} ({delta}) => {pos}')
    print(-pos[0]*pos[2])

if __name__ == "__main__":
    solve(fileinput.input(sys.argv[1]))
