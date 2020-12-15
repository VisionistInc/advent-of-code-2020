#!/usr/bin/env python

mem = {}
mask_or = 0
mask_and = 0b111111111111111111111111111111111111

with open("input") as infile:
    for line in infile:
        if line[:4] == "mask":
            line = line.split(" = ")[1]
            mask_or = int(line.replace("X", "0"), 2)
            mask_and = int(line.replace("X", "1"), 2)
        elif line[:3] == "mem":
            addr, val = line[4:].split("] = ")
            addr, val = int(addr), int(val)
            mem[addr] = (val & mask_and) | mask_or

total = 0
for addr in mem:
    total += mem[addr]

print(total)
