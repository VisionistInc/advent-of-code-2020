#!/usr/bin/env python3

with open("input") as infile:
    instructions = [line.strip() for line in infile]


# 0 = north
# 1 = east
# 2 = south
# 3 = west

direction = 1

location_x = 0
location_y = 0

# Action N means to move north by the given value.
# Action S means to move south by the given value.
# Action E means to move east by the given value.
# Action W means to move west by the given value.
# Action L means to turn left the given number of degrees.
# Action R means to turn right the given number of degrees.
# Action F means to move forward by the given value in the direction the ship is currently facing.


for instruction in instructions:
    inst = instruction[0]
    val = int(instruction[1:])

    if inst == "N":
        location_y += val
    elif inst == "S":
        location_y -= val
    elif inst == "E":
        location_x += val
    elif inst == "W":
        location_x -= val
    elif inst == "L":
        direction -= int(val / 90)
        direction %= 4
    elif inst == "R":
        direction += int(val / 90)
        direction %= 4
    elif inst == "F":
        x_dif, y_dif = [[0, val], [val, 0], [0, -val], [-val, 0]][direction]
        location_x += x_dif
        location_y += y_dif

print(location_x, location_y)
print(abs(location_x) + abs(location_y))
