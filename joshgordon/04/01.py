#!/usr/bin/env python3

passports = []

with open("input") as infile:
    passport = []
    for line in infile:
        if line.strip() == "":
            passports.append(passport)
            passport = []
            continue
        passport.extend(line.strip().split(" "))
    passports.append(passport)

passports = [
    {txt.split(":")[0]: txt.split(":")[1] for txt in passport} for passport in passports
]

count_valid = 0

# byr (Birth Year)
# iyr (Issue Year)
# eyr (Expiration Year)
# hgt (Height)
# hcl (Hair Color)
# ecl (Eye Color)
# pid (Passport ID)
# cid (Country ID)


for passport in passports:
    if (
        "byr" in passport
        and "iyr" in passport
        and "eyr" in passport
        and "hgt" in passport
        and "hcl" in passport
        and "ecl" in passport
        and "pid" in passport
    ):
        count_valid += 1
print(count_valid)
