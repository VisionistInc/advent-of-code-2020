#!/usr/bin/env python

from functools import cache

adapters = []

with open("input") as infile:
    adapters = [int(line.strip()) for line in infile]

adapters = sorted(adapters)
goal = max(adapters) + 3

# states = [(0,)]
# fullstates = []

# This is *technically* correct but awful and slow. :(
# while states:
#     state = states.pop(0)
#     for adapter in adapters:
#         if adapter > state[-1] and adapter <= state[-1] + 3:
#             # valid adapter so let's push a new state:
#             st = list(state)
#             st.append(adapter)
#             if adapter + 3 == goal:
#                 fullstates.append(tuple(st))
#                 print(".", end="")
#             else:
#                 states.append(tuple(st))
#         elif adapter > state[-1] + 3:
#             # if we're past the highest adapter(because hey, they're sorted!) we can skip the rest.
#             break
# print(len(fullstates))

################################################################################
# Let's try something new.
################################################################################

# you can't do recursion!
# haha @cache go brrrr!
@cache
def count_paths(current_position):
    if current_position + 3 == goal:
        return 1

    total_paths = 0
    for i in range(1, 4):
        if current_position + i in adapters:
            total_paths += count_paths(current_position + i)

    return total_paths


print(count_paths(0))
