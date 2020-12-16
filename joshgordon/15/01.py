#!/usr/bin/env python3

with open("input") as infile:
    sequence = [int(num) for num in infile.read().strip().split(",")]

while len(sequence) != 2020:
    last = sequence[-1]
    if last not in sequence[:-1]:
        sequence.append(0)
    else:
        sequence.append(list(reversed(sequence[:-1])).index(last) + 1)

print(sequence[-1])
