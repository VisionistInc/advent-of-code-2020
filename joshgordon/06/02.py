#!/usr/bin/env python3

forms = []

with open("input") as infile:
    form = None
    for line in infile:
        line = line.strip()
        if line == "":
            forms.append(form)
            form = None
            continue
        if form == None:
            form = set(line)
        else:
            form &= set(line)
    forms.append(form)

print(sum([len(lst) for lst in forms]))
