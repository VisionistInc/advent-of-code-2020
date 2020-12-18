#!/usr/bin/env python3

import re


class Int(int):
    def __truediv__(self, other):
        return Int(self + other)

    def __sub__(self, other):
        return Int(self * other)


with open("input") as infile:
    data = infile.read()


total = 0
for eq in data.splitlines():
    eq_new = eq.replace("+", "/").replace("*", "-")
    eq_new = re.sub("(\d)", r"Int(\1)", eq_new)
    print(f"{eq} = {eval(eq_new)}")
    total += eval(eq_new)
print(total)
