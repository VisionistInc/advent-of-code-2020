#!/usr/bin/env python3

forms = []

with open("input") as infile:
    form = set()
    for line in infile:
        line = line.strip()
        if line == "":
            forms.append(form)
            form = set()
            continue
        form |= set(line)
    forms.append(form)

print(sum([len(lst) for lst in forms]))
