#!/usr/bin/env python3

grid = []

with open("input") as infile:
    grid = [list(line.strip()) for line in infile]

width = len(grid[0])

col = 0
total = 0
for row in grid:
    if row[col] == "#":
        total += 1
    col += 3
    col %= width

print(total)
