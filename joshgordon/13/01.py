#!/usr/bin/env python3


with open("input") as infile:
    start_time, trains = [line.strip() for line in infile]

trains = [int(train) for train in trains.split(",") if train != "x"]
start_time = int(start_time)

lowest_train_num = trains[0]

for train in trains:
    if (train - (start_time % train)) < (
        lowest_train_num - (start_time % lowest_train_num)
    ):
        lowest_train_num = train

print("Next train: ", lowest_train_num)
print("Start Time: ", start_time)
print("Time to wait??? ", lowest_train_num - (start_time % lowest_train_num))
print("Time since last train? ", start_time % lowest_train_num)
# time to wait is... lowest_train_num - (start_time % lowest_train_num)
print(lowest_train_num * (lowest_train_num - (start_time % lowest_train_num)))
