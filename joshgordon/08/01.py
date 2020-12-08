#!/usr/bin/env python3

instructions = []
instruction_pointer = 0
acc = 0

visited_instructions = set()

with open("input") as infile:
    for line in infile:
        instruction = line.strip().split(" ")
        instruction[1] = int(instruction[1])
        instructions.append(instruction)


while instruction_pointer not in visited_instructions:
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

print(acc)
