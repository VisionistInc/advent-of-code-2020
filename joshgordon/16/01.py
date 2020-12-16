#!/usr/bin/env python3

rules = {}
my_ticket = []
all_tickets = []

current = 0

with open("input") as infile:
    for line in infile:
        if line.strip() == "":
            continue
        elif line.strip() == "your ticket:":
            current = 1
            continue
        elif line.strip() == "nearby tickets:":
            current = 2
            continue

        if current == 0:
            field, values = line.strip().split(":")
            values = values.strip()
            values = values.split(" or ")
            values = [[int(num) for num in rule.split("-")] for rule in values]
            rules[field] = values
        elif current == 1:
            my_ticket = [int(val) for val in line.strip().split(",")]

        elif current == 2:
            all_tickets.append([int(val) for val in line.strip().split(",")])

invalid = 0
for ticket in all_tickets:
    for value in ticket:
        for rule in rules:
            min_1, max_1 = rules[rule][0]
            min_2, max_2 = rules[rule][1]

            if value >= min_1 and value <= max_1 or value >= min_2 and value <= max_2:
                break
        else:
            invalid += value


print(invalid)
