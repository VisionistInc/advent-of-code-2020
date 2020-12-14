#!/usr/bin/env python3

# STAND BACK: We're going to do _MATH_
import math

with open("input") as infile:
    start_time, trains = [line.strip() for line in infile]


trains = [
    (idx, int(train)) for idx, train in enumerate(trains.split(",")) if train != "x"
]

# Chinese remainder theorm
# sum of the products:
#     product of remainder (in our case, (bus number - bus position)) mod the bus number
#       and the product of the other remainders / the local mod
#       and the inverse of the product of the other remainders (mod the original mod, so bus number)


def compute_inverse(num, mod):
    i = 1
    while num * i % mod != 1:
        i += 1
    return i


product = math.prod([train[1] for train in trains])

remainders = sum(
    [
        math.prod(
            [
                (train[1] - train[0]) % train[1],
                product // train[1],
                compute_inverse(product // train[1], train[1]),
            ]
        )
        for train in trains
    ]
)
print(remainders % product)
