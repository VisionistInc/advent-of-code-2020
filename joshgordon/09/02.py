#!/usr/bin/env python3

import sys

preamble_length = 25

with open("input") as infile:
    indata = [int(line.strip()) for line in infile]


def check_values(i, indata):
    for j in range(i - preamble_length, i - 1):
        for k in range(j + 1, i):
            if indata[i] == indata[j] + indata[k]:
                return True
    return False


invalid_number = None

for i in range(preamble_length, len(indata)):
    if not check_values(i, indata):
        invalid_number = indata[i]
        break

# part 2 time...
for i in range(len(indata) - 1):
    for j in range(i, len(indata)):
        if sum(indata[i:j]) == invalid_number:
            print(f"sum(indata[{i}:{j}]) == {invalid_number}")
            max_num = max(indata[i:j])
            min_num = min(indata[i:j])
            print(f"max + min = {max_num + min_num}")
            sys.exit(0)
