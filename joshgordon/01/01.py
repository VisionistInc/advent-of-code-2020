#!/usr/bin/env python3

import sys

nums = []

with open("input") as infile:
    nums = [int(num) for num in infile]

for idx, num in enumerate(nums):
    for num2 in nums[idx:]:
        if num + num2 == 2020:
            print(num * num2)
            sys.exit(0)

sys.exit(1)
