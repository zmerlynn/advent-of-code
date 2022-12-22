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
    message = [ (message[i] * 811589153, i) for i in range(message_len) ]
    mix_order = message[:]

    for _ in range(10):
        message = mix(message, mix_order)

    message = [ n for n, _ in message ]
    print(message)
    start = message.index(0)
    print(message[(start+1000) % message_len]+message[(start+2000) % message_len]+message[(start+3000) % message_len])

def mix(message, mix_order):
    message_len = len(message)

    for num, orig_index in mix_order:
        i = message.index( (num, orig_index) )

        old_message = message[:]
        minus_num = message[:i] + message[i+1:]

        j = (i + num) % (message_len - 1)
        message = minus_num[:j] + [ (num, orig_index) ] + minus_num[j:]

        # print(f"old={[ n for n, _ in old_message ]}, m[i]={num}, i={i}, j={j} => message={[ n for n, _ in message ]}")
        # print(f"m[i]={num}, i={i}, j={j}")

    assert sorted(message) == sorted(mix_order)
    return message

def tests():
    print("--- tests done ----")

if __name__ == "__main__":
    tests()
    solve(fileinput.input(sys.argv[1]))
