#!/usr/bin/env python

import copy

grid = []

with open("input") as infile:
    grid = [list(line.strip()) for line in infile]

x_size = len(grid[0])
y_size = len(grid)

old_grid = None

iterations = 0

while grid != old_grid:
    old_grid = copy.deepcopy(grid)

    iterations += 1
    print(f"\rIteration: {iterations}", end="")

    for row in range(y_size):
        for col in range(x_size):
            if old_grid[row][col] == ".":
                continue

            adjacent_filled = 0
            for row_offset in range(row - 1, row + 2):
                if adjacent_filled >= 4:
                    continue
                for col_offset in range(col - 1, col + 2):
                    if adjacent_filled >= 4:
                        continue
                    if row_offset < 0 or row_offset >= y_size:
                        continue
                    if col_offset < 0 or col_offset >= x_size:
                        continue
                    if row_offset == row and col_offset == col:
                        continue

                    if old_grid[row_offset][col_offset] == "#":
                        adjacent_filled += 1

            if old_grid[row][col] == "L" and adjacent_filled == 0:
                grid[row][col] = "#"
            if old_grid[row][col] == "#" and adjacent_filled >= 4:
                grid[row][col] = "L"

print()

total_filled = 0
for row in grid:
    for col in row:
        if col == "#":
            total_filled += 1
print(f"Seats filled: {total_filled}")
