#!/usr/bin/env python3

with open("input") as infile:
    instructions = [line.strip() for line in infile]


# 0 = north
# 1 = east
# 2 = south
# 3 = west

location_x = 0
location_y = 0

# Action N means to move the waypoint north by the given value.
# Action S means to move the waypoint south by the given value.
# Action E means to move the waypoint east by the given value.
# Action W means to move the waypoint west by the given value.
# Action L means to rotate the waypoint around the ship left (counter-clockwise) the given number of degrees.
# Action R means to rotate the waypoint around the ship right (clockwise) the given number of degrees.
# Action F means to move forward to the waypoint a number of times equal to the given value.

waypoint_x = 10
waypoint_y = 1

for instruction in instructions:
    inst = instruction[0]
    val = int(instruction[1:])

    # print(inst, val)

    if inst == "N":
        waypoint_y += val
    elif inst == "S":
        waypoint_y -= val
    elif inst == "E":
        waypoint_x += val
    elif inst == "W":
        waypoint_x -= val
    # to rotate left: x = -y; y = x
    elif inst == "L":
        for i in range(int(val / 90)):
            waypoint_x, waypoint_y = -waypoint_y, waypoint_x
    # to rotate right: x= = y, y = -x
    elif inst == "R":
        for i in range(int(val / 90)):
            waypoint_x, waypoint_y = waypoint_y, -waypoint_x
    elif inst == "F":
        location_x += waypoint_x * val
        location_y += waypoint_y * val

    # print("Location: ", location_x, location_y)
    # print("Waypoint: ", waypoint_x, waypoint_y)

print(abs(location_x) + abs(location_y))
