#!/usr/bin/env python3


with open("input") as infile:
    sequence = [int(num) for num in infile.read().strip().split(",")]

sequence_hash = {int(num): idx for idx, num in enumerate(sequence[:-1])}

position = len(sequence) - 1
last = sequence[-1]

# print(sequence_hash)
while position != 29999999:
    # print(last)
    # print(sequence_hash)
    if last not in sequence_hash:
        # print(f"{last} not in sequence_hash")
        last_new = 0
    else:
        last_new = position - sequence_hash[last]
    sequence_hash[last] = position
    last = last_new
    position += 1

print(last)
