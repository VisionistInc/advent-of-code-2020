#!/usr/bin/env python3

from collections import defaultdict
import copy

universe = defaultdict(lambda: defaultdict(lambda: defaultdict(lambda: ".")))

with open("input") as infile:
    for y, line in enumerate(infile):
        for x, value in enumerate(line.strip()):
            universe[x][y][0] = value

max_dim_x = len(universe)
max_dim_y = len(universe[0])
max_dim_z = len(universe[0][0])

range_x = (0, max_dim_x)
range_y = (0, max_dim_y)
range_z = (0, max_dim_z)
print(universe)
print(range_x, range_y, range_z)


# let's iterate this sucker....

neighbors = [
    [-1, -1, -1],
    [-1, -1, 0],
    [-1, -1, 1],
    [-1, 0, -1],
    [-1, 0, 0],
    [-1, 0, 1],
    [-1, 1, -1],
    [-1, 1, 0],
    [-1, 1, 1],
    [0, -1, -1],
    [0, -1, 0],
    [0, -1, 1],
    [0, 0, -1],
    [0, 0, 1],
    [0, 1, -1],
    [0, 1, 0],
    [0, 1, 1],
    [1, -1, -1],
    [1, -1, 0],
    [1, -1, 1],
    [1, 0, -1],
    [1, 0, 0],
    [1, 0, 1],
    [1, 1, -1],
    [1, 1, 0],
    [1, 1, 1],
]

# 6 cycles:
for i in range(6):

    old_universe = copy.deepcopy(universe)
    range_x = (range_x[0] - 1, range_x[1] + 1)
    range_y = (range_y[0] - 1, range_y[1] + 1)
    range_z = (range_z[0] - 1, range_z[1] + 1)

    for x in range(range_x[0], range_x[1]):
        for y in range(range_y[0], range_y[1]):
            for z in range(range_x[0], range_z[1]):
                # let's get the number of active neighbors:
                active_neighbors = 0
                for neighbor in neighbors:
                    if (
                        old_universe[x + neighbor[0]][y + neighbor[1]][z + neighbor[2]]
                        == "#"
                    ):
                        active_neighbors += 1

                if universe[x][y][z] == "#" and not (
                    active_neighbors == 2 or active_neighbors == 3
                ):
                    universe[x][y][z] = "."
                elif universe[x][y][z] == "." and active_neighbors == 3:
                    universe[x][y][z] = "#"


active = 0

for x in range(range_x[0], range_x[1]):
    for y in range(range_y[0], range_y[1]):
        for z in range(range_x[0], range_z[1]):
            if universe[x][y][z] == "#":
                active += 1

print(active)
