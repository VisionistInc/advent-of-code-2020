#!/usr/bin/env python3

preamble_length = 25

with open("input") as infile:
    indata = [int(line.strip()) for line in infile]


def check_values(i, indata):
    for j in range(i - preamble_length, i - 1):
        for k in range(j + 1, i):
            if indata[i] == indata[j] + indata[k]:
                return True
    return False


for i in range(preamble_length, len(indata)):
    if not check_values(i, indata):
        invalid_number = indata[i]
        print(f"invalid: {indata[i]}")
        break
