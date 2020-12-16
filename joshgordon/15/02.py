#!/usr/bin/env python3

with open("input") as infile:
    sequence = [int(num) for num in infile.read().strip().split(",")]

while len(sequence) != 30000000:
    if len(sequence) % 1000 == 0:
        print(f"\r{len(sequence)}", end="")
    last = sequence[-1]
    if last not in sequence[:-1]:
        sequence.append(0)
    else:
        sequence.append(list(reversed(sequence[:-1])).index(last) + 1)
print()

print(sequence[-1])
