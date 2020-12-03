#!/usr/bin/env python3

with open("input") as infile:

    passwords = [line.strip().split(": ") for line in infile]

num_valid = 0
for password in passwords:
    rnge = [int(n) for n in password[0].split(" ")[0].split("-")]
    char = password[0].split(" ")[1]

    count = password[1].count(char)
    if rnge[0] <= count <= rnge[1]:
        num_valid += 1

print(num_valid)
