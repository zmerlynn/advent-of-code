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

def solve(inp):
    message = [ int(l.strip()) for l in inp ]
    message_len = len(message)
    mix_order = message[:]
    message = [ (n, False) for n in message ] # tuple of (num, mixed?)
    print(message_len)

    for num in mix_order:
        i = message.index( (num, False) )

        old_message = message[:]
        minus_num = message[:i] + message[i+1:]

        j = (i + num) % (message_len - 1)
        message = minus_num[:j] + [ (num, True) ] + minus_num[j:]

        # print(f"old={old_message}, m[i]={num}, i={i}, j={j} => message={message}")
        print(f"m[i]={num}, i={i}, j={j}")

    assert sorted(message) == sorted([ (n, True) for n in mix_order ])
    start = message.index( (0, True) )
    print(message[(start+1000) % message_len][0]+message[(start+2000) % message_len][0]+message[(start+3000) % message_len][0])

def tests():
    print("--- tests done ----")

if __name__ == "__main__":
    tests()
    solve(fileinput.input(sys.argv[1]))
