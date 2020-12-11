#!/usr/bin/env python

adapters = []

with open("input") as infile:
    adapters = [int(line.strip()) for line in infile]


adapters = sorted(adapters)
print(adapters)

num_1_jumps = 0
num_3_jumps = 1

previous = 0

for adapter in adapters:
    if adapter - previous == 1:
        num_1_jumps += 1
    elif adapter - previous == 3:
        num_3_jumps += 1
    elif adapter - previous == 2:
        print("Found one off by 2!")
    else:
        print("WTF", adapter - previous)

    previous = adapter

print(num_1_jumps, num_3_jumps)
print(f"Part 1 answer: {num_1_jumps * num_3_jumps}")
