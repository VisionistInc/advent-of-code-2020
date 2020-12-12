#!/usr/bin/env python

import copy
from functools import cache


grid = []

with open("input") as infile:
    grid = [list(line.strip()) for line in infile]

x_size = len(grid[0])
y_size = len(grid)

old_grid = None

iterations = 0


directions = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)]


@cache
def multiply_direction(direction, mult):
    return [mult * i for i in direction]


while grid != old_grid:
    old_grid = copy.deepcopy(grid)

    iterations += 1
    print(f"\rIteration: {iterations}", end="")

    for row in range(y_size):
        for col in range(x_size):
            if old_grid[row][col] == ".":
                continue

            adjacent_filled = 0
            for direction in directions:
                if adjacent_filled > 5:
                    continue
                for i in range(1, max(x_size, y_size)):
                    col_offset, row_offset = multiply_direction(direction, i)

                    if row + row_offset < 0 or row + row_offset >= y_size:
                        break
                    if col + col_offset < 0 or col + col_offset >= x_size:
                        break

                    # skip the empty floor
                    if old_grid[row + row_offset][col + col_offset] == ".":
                        continue

                    # seat filled - add 1 to adjacency and stop looking
                    if old_grid[row + row_offset][col + col_offset] == "#":
                        adjacent_filled += 1
                        break

                    # empty seat, stop looking.
                    if old_grid[row + row_offset][col + col_offset] == "L":
                        break

            if old_grid[row][col] == "L" and adjacent_filled == 0:
                grid[row][col] = "#"
            if old_grid[row][col] == "#" and adjacent_filled >= 5:
                grid[row][col] = "L"

print()

total_filled = 0
for row in grid:
    for col in row:
        if col == "#":
            total_filled += 1
print(f"Seats filled: {total_filled}")
