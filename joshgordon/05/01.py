#!/usr/bin/env python3


with open("input") as infile:
    boardings = [line.strip() for line in infile]

print(boardings)

positions = []
for pas in boardings:
    pas = pas.replace("F", "0")
    pas = pas.replace("B", "1")
    pas = pas.replace("R", "1")
    pas = pas.replace("L", "0")

    positions.append(int(pas, 2))

print(max(positions))
