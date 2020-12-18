#!/usr/bin/env python3

from enum import Enum


class Oper(Enum):
    MULT = "*"
    ADD = "+"


with open("input") as infile:
    problems = [line.strip() for line in infile]

# this is ugly, and I would never ever consider putting this into production anywhere.
# debugging is a nightmare that I would not wish upon my worst enemy.

# returns a vale and the remainder of the token stream to return
def handle_parens(tokens):
    val = None
    operator = None
    tokens = tokens.copy()
    while len(tokens) > 0:
        t_val = None
        if tokens[0] == "(":
            t_val, rest, _ = handle_parens(tokens[1:])
            tokens = rest
        elif tokens[0] == ")":
            return val, tokens[1:], True

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
            t_val, tokens, close_paren = handle_parens([t_val] + tokens)
            val *= t_val
            if close_paren:
                return val, tokens, True

    return val, [], False


total_sum = 0

for problem in problems:
    print(problem, end=" ")
    problem = list(problem.replace(" ", ""))
    ans = handle_parens(problem)[0]
    print("=", ans)
    total_sum += ans

print(total_sum)
