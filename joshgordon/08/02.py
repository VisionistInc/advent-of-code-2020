#!/usr/bin/env python3

import copy

instructions = []


class InfiniteLoopException(Exception):
    pass


def computer(instructions):
    instruction_pointer = 0
    acc = 0

    visited_instructions = set()

    while instruction_pointer not in visited_instructions and instruction_pointer < len(
        instructions
    ):
        visited_instructions.add(instruction_pointer)

        inst, arg = instructions[instruction_pointer]

        if inst == "nop":
            instruction_pointer += 1
            continue

        if inst == "acc":
            acc += arg
            instruction_pointer += 1
            continue

        if inst == "jmp":
            instruction_pointer += arg
            continue

    if instruction_pointer in visited_instructions:
        raise InfiniteLoopException
    else:
        return acc


with open("input") as infile:
    for line in infile:
        instruction = line.strip().split(" ")
        instruction[1] = int(instruction[1])
        instructions.append(instruction)

for i in range(len(instructions)):
    if instructions[i][0] == "jmp" or instructions[i][0] == "nop":
        inst_copy = copy.deepcopy(instructions)
        if inst_copy[i][0] == "jmp":
            inst_copy[i][0] = "nop"
        else:
            inst_copy[i][0] = "jmp"

        try:
            solution = computer(inst_copy)
            print(f"Solution Found: {solution}")
        except InfiniteLoopException:
            continue
