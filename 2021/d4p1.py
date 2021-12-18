#!/usr/bin/python3

import collections
import fileinput
import re
import sys

SIZE=5

def solve(inp):
    calls = next(inp)
    calls = [ int(call) for call in calls.strip().split(',') ]

    boards = []
    while True:
        try: 
            boards.append(read_board(inp))
        except StopIteration:
            break

    positions = call_positions(boards)

    marks = [ [ [ False for r in range(SIZE) ] for c in range(SIZE) ]  for b in range(len(boards)) ]

    winner = None
    for call in calls:
        mark_boards(marks, positions, call)
        for board in range(len(boards)):
            if winner_winner_chicken_dinner(marks[board]):
                winner = (board, call)
                break
        if winner:
            break

    board, call = winner
    board_marks = marks[board]
    board = boards[board]
    sum = 0
    for r in range(SIZE):
        for c in range(SIZE):
            if not board_marks[r][c]:
                sum += board[r][c]
    print(sum*call)

def read_board(inp):
    next(inp)
    board = []
    for i in range(SIZE):
        row = next(inp)
        row = [ int(x) for x in row.split(' ') if x.strip() != '' ]
        board.append(row)
    return board

# returns call:int -> [ (board, row, col), (board, row, col), ... ]
def call_positions(boards):
    pos = collections.defaultdict(list)
    for b in range(len(boards)):
        for r in range(SIZE):
            for c in range(SIZE):
                pos[ boards[b][r][c] ].append( (b,r,c) )
    return pos

def mark_boards(marks, positions, call):
    for board, row, col in positions[call]:
        marks[board][row][col] = True

def winner_winner_chicken_dinner(board_marks):
    cols = [True] * 5
    for row in board_marks:
        if all(row):
            return True
        for col in range(SIZE):
            cols[col] = cols[col] and row[col]
    return any(cols)

if __name__ == "__main__":
    solve(fileinput.input(sys.argv[1]))
