#!/usr/bin/env python3

import re

bag_re = re.compile(r"(\w+ \w+) bags contain (.*)")
contains_re = re.compile(r"(\d) (\w+ \w+)")

bags = {}


class Bag:
    color = ""
    contains = None

    def __init__(self, color):
        self.color = color
        self.contains = {}  # map from color_name to count of bags

    def can_contain(self, color):
        if color in self.contains:
            return True
        for bag in self.contains:
            if bags[bag].can_contain(color):
                return True
        return False

    def count_children(self, recurse=False):
        total = 0
        if recurse:
            total = 1
        for bag in self.contains:
            total += self.contains[bag] * bags[bag].count_children(recurse=True)
        return total

    def __repr__(self):
        if len(self.contains) > 0:
            bag_contains = ", ".join(
                [
                    f"{self.contains[color]} {color} bag{'s' if self.contains[color] != 1 else ''}"
                    for color in self.contains
                ]
            )

            return f"{self.color} bags contain {bag_contains}."
        else:
            return f"{self.color} bags contain no other bags."


with open("input") as infile:
    for line in infile:
        # BLARGH

        line = line.strip()
        match = bag_re.match(line)
        groups = match.groups()

        this_bag = Bag(groups[0])

        this_contains = groups[1]
        contains = []
        if this_contains != "no other bags.":
            contains = this_contains.strip(".").split(", ")
            for bag in contains:
                groups = contains_re.match(bag).groups()
                this_bag.contains[groups[1]] = int(groups[0])
        bags[this_bag.color] = this_bag

    print(bags["shiny gold"].count_children())
