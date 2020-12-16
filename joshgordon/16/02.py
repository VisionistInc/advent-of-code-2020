#!/usr/bin/env python3

rules = {}
my_ticket = []
all_tickets = []

current = 0


def is_field_valid(rule, value):
    min_1, max_1 = rule[0]
    min_2, max_2 = rule[1]

    return value >= min_1 and value <= max_1 or value >= min_2 and value <= max_2


def is_valid_ticket(ticket):
    for value in ticket:
        valid = sum([is_field_valid(rules[rule], value) for rule in rules])
        if valid == 0:
            return False
    return True


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

# filter out invalid tickets
all_tickets = [ticket for ticket in all_tickets if is_valid_ticket(ticket)]


# start assuming any field could be any value...
possible_fields = []
for field in my_ticket:
    possible_fields.append(set(rules))

# go through all of the tickets and weed out invalid fields from the possible tickets
for ticket in [my_ticket] + all_tickets:
    for val_idx, val in enumerate(ticket):
        for name, rule in rules.items():
            # if this ticket isn't valid for the given rule, take it out as a potential option
            if not is_field_valid(rule, val):
                possible_fields[val_idx].discard(name)

# Now we've eliminated the ones that definitely don't work, but some of them still could work in more than one
# place... let's fix that.

final_values = [None] * len(possible_fields)

# Keep going as long as all the fields aren't set.
while None in final_values:
    for idx, fieldset in enumerate(possible_fields):
        # if there's only one possible value for a given field, set it as the final result and remove it from
        # all the other possible values for the rest of the fields.
        if len(fieldset) == 1:
            val = list(fieldset)[0]
            final_values[idx] = val
            for f in possible_fields:
                f.discard(val)

# turn our ticket into a dict and find the final answer.
my_ticket = dict(zip(final_values, my_ticket))
product = 1
for field, val in my_ticket.items():
    if "departure" in field:
        product *= val
print(product)
