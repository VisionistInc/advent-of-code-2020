#!/usr/bin/env python3

import re
import sys

if len(sys.argv) == 1:
    input_filename = "input"
else:
    input_filename = sys.argv[1]

passports = []

with open(input_filename) as infile:
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

# byr (Birth Year)
# iyr (Issue Year)
# eyr (Expiration Year)
# hgt (Height)
# hcl (Hair Color)
# ecl (Eye Color)
# pid (Passport ID)
# cid (Country ID)

valid = [
    passport
    for passport in passports
    if (
        "byr" in passport
        and "iyr" in passport
        and "eyr" in passport
        and "hgt" in passport
        and "hcl" in passport
        and "ecl" in passport
        and "pid" in passport
    )
]

print(len(valid))


def validate(passport):
    byr = int(passport["byr"])
    if byr < 1920 or byr > 2002:
        print(f"byr {byr} invalid")
        return False

    iyr = int(passport["iyr"])
    if iyr < 2010 or iyr > 2020:
        print(f"iyr {iyr} invalid")
        return False

    eyr = int(passport["eyr"])
    if eyr < 2020 or eyr > 2030:
        print(f"eyr {eyr} invalid")
        return False

    hgt_re = re.compile(r"^(\d+)(in|cm)$")
    match = hgt_re.match(passport["hgt"])

    if not match:
        return False
        print(f"hgt {hgt} invalid")
    hgt = int(match.groups()[0])

    if match.groups()[1] == "cm":
        if hgt < 150 or hgt > 193:
            print(f"hgt {hgt}{match.groups()[0]} invalid")
            return False
    elif match.groups()[1] == "in":
        if hgt < 59 or hgt > 76:
            print(f"hgt {hgt}{match.groups()[0]} invalid")
            return False

    hcl_re = re.compile(r"^#[0-9a-f]{6}$")
    if not hcl_re.match(passport["hcl"]):
        print(f"hcl {passport['hcl']} invalid")
        return False

    valid_ecl = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
    if passport["ecl"] not in valid_ecl:
        print(f"ecl {passport['ecl']} invalid")
        return False

    pid = passport["pid"]
    pid_re = re.compile(r"^\d{9}$")
    if not pid_re.match(pid):
        print(f"pid {pid} invalid")
        return False

    return True


valid = [passport for passport in valid if validate(passport)]
print(len(valid))
