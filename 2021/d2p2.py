#!/usr/bin/python3

import fileinput
import sys

def solve(inp):
    x = 0
    d = 0
    aim = 0
    for line in inp:
        cmd, arg = line.split()
        arg = int(arg)
        if cmd == "down":
            aim += arg
        elif cmd == "up":
            aim -= arg
        elif cmd == "forward":
            x += arg
            d += arg * aim
        else:
            raise Exception()

        print(f'{cmd} {arg} => x={x}, d={d}, aim={aim}')
    print(x*d)

if __name__ == "__main__":
    solve(fileinput.input(sys.argv[1]))
