#!/usr/bin/env python3


with open("input") as infile:
    boardings = [line.strip() for line in infile]

positions = []
for pas in boardings:
    pas = pas.replace("F", "0")
    pas = pas.replace("B", "1")
    pas = pas.replace("R", "1")
    pas = pas.replace("L", "0")

    positions.append(int(pas, 2))

for i in range(max(positions)):
    if i not in positions and i + 1 in positions and i - 1 in positions:
        print(i)
        break
