#!/usr/bin/env python3

grid = []

with open("input") as infile:
    grid = [list(line.strip()) for line in infile]

# slope is (x, y)
def compute(grid, slope):

    width = len(grid[0])

    col = 0
    total = 0
    for row in range(0, len(grid), slope[1]):
        if grid[row][col] == "#":
            total += 1
        col += slope[0]
        col %= width
    return total


slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
product = 1
for slope in slopes:
    product *= compute(grid, slope)

print(product)
