#!/usr/bin/env python

mem = {}
mask_or = 0
mask = ""

with open("input") as infile:
    for line in infile:
        if line[:4] == "mask":
            # for "0" we leave alone
            # for "1" we set bit high (always)
            # for "X" we do all possible combos of high and low.
            line = line.split(" = ")[1]

            # for "1" we'll just set an or-mask again
            mask_or = int(line.replace("X", "0"), 2)

            # for the "X" we'll set all the 1's to 0 (because we deal with that in the or-mask) and move on
            mask = list(line.replace("1", "0"))

        elif line[:3] == "mem":
            addr, val = line[4:].split("] = ")
            addr, val = int(addr), int(val)

            # go ahead and set the bits high from the mask
            addr |= mask_or

            # iterate over all possible combos of bits for "X" values
            for i in range(2 ** mask.count("X")):
                new_mask = []
                num_replaced = 0

                # loop thorugh the mask and replace "X" with an appropriate bit.
                for char in mask:
                    if char == "X":
                        new_mask.append(str((i >> num_replaced) & 1))
                        num_replaced += 1

                    # if it's not an "X" just pass it through as-is.
                    else:
                        new_mask.append(char)

                # convert the mask from a list to an int
                new_mask = int("".join(new_mask), 2)
                # set the new value. Since we're ultimately setting both, we can just use XOR to flip the bits
                # that need to change.
                mem[addr ^ new_mask] = val

total = 0
for addr in mem:
    total += mem[addr]

print(total)
