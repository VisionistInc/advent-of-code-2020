#!/usr/bin/env python3

from enum import Enum


class Oper(Enum):
    MULT = "*"
    ADD = "+"


with open("input") as infile:
    problems = [line.strip() for line in infile]
print(problems)

# returns a vale and the remainder of the token stream to return
def handle_parens(tokens):
    val = None
    operator = None
    tokens = tokens.copy()
    while len(tokens) > 0:
        t_val = None
        if tokens[0] == "(":
            t_val, rest = handle_parens(tokens[1:])
            tokens = rest
        elif tokens[0] == ")":
            return val, tokens[1:]

        elif tokens[0] == "*":
            operator = Oper.MULT
            tokens = tokens[1:]
        elif tokens[0] == "+":
            operator = Oper.ADD
            tokens = tokens[1:]

        # if we get to this point we've got a number.
        else:
            t_val = int(tokens[0])
            tokens = tokens[1:]

        if operator is None and t_val is not None:
            val = t_val
        elif operator == Oper.ADD and t_val is not None:
            val += t_val
        elif operator == Oper.MULT and t_val is not None:
            val *= t_val

    return val, []


total_sum = 0

for problem in problems:
    problem = list(problem.replace(" ", ""))
    total_sum += handle_parens(problem)[0]

print(total_sum)
