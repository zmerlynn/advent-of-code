#!/usr/bin/python3

import collections
import fileinput
import functools
import itertools
import math
import re
import sys

def solve(inp):
    algo = tuple([ bit(c) for c in next(inp).strip() ])
    next(inp)

    row = 0
    r = InfiniteRaster()
    for l in inp:
        col = 0
        for c in l.strip():
            if bit(c):
                r.on(row, col)
            col += 1
        row += 1

    for i in range(50):
        r = r.enhance(algo)
        print(i, len(r._points))

class InfiniteRaster(object):
    def __init__(self):
        self._points = set()
        self._bounds = None # kept as ([min_row, max_row),[min_col,max_col))
        self._void = False # is the void on or off?

    def copy(self):
        r = InfiniteRaster()
        r._points = self._points.copy()
        r._bounds = self._bounds
        return r

    def on(self, row, col):
        self._points.add( (row, col) )

        if not self._bounds:
            self._bounds=((row,row+1),(col,col+1))
        else:
            ((min_row,max_row),(min_col,max_col))=self._bounds
            self._bounds=((min(row, min_row), max(row+1, max_row)), (min(col, min_col), max(col+1, max_col)))

    def pretty(self):
        ((min_row,max_row),(min_col,max_col))=self._bounds
        for row in range(min_row, max_row):
            row_str = ""
            for col in range(min_col, max_col):
                row_str += "#" if (row,col) in self._points else "."
            print(row_str)
        print("void =", self._void)
        print("bounds = ", self._bounds)

    # returns a new raster
    def enhance(self, algo):
        assert(len(algo) == 512)
        new_raster = InfiniteRaster()

        # We only ever need to contemplate 1 more pixel
        ((min_row,max_row),(min_col,max_col))=self._bounds
        ((min_row,max_row),(min_col,max_col))=((min_row-1,max_row+1),(min_col-1,max_col+1))

        for row in range(min_row, max_row):
            for col in range(min_col, max_col):
                if self._enhanced_pixel(algo, row, col):
                    new_raster.on(row,col)

        if self._void:
            new_raster._void = algo[511]
        else:
            new_raster._void = algo[0]

        return new_raster

    def _enhanced_pixel(self, algo, row, col):
        bit_str = ""
        for r in range(row-1, row+2):
            for c in range(col-1, col+2):
                bit_str += "1" if self.get(r,c) else "0"
        return algo[int(bit_str, 2)]

    def get(self, row, col):
        ((min_row,max_row),(min_col,max_col))=self._bounds
        if row < min_row or row >= max_row or col < min_col or col >= max_col:
            return self._void

        return (row, col) in self._points


def bit(c):
    return 1 if c == "#" else 0

if __name__ == "__main__":
    solve(fileinput.input(sys.argv[1]))
