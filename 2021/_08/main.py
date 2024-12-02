#!/usr/bin/env python3

import os
import sys
import random
from typing import Dict, List

input_path = sys.argv[1] if len(sys.argv) > 1 else "input.example"
if not os.path.exists(input_path):
    print(f"Given input {input_path} not found")
    sys.exit(1)

print(f"> working with {input_path}")

# === [ part one ] ============================================
with open(input_path) as fh:
    outputs: List = [*map(lambda line: line.strip().split(" | ")[1].split(" "), [*filter(lambda line: line != "", fh.readlines())])]

# filter out the easy numbers
outputs = [*map(lambda output: [*filter(lambda o: o in [2, 4, 3, 7], map(lambda o: len(o), output))], outputs)]

print("[part one] the number of outputs with easy digits is", sum([len(o) for o in outputs]))

# === [ part two ] ============================================
with open(input_path) as fh:
    outputs = []
    for line in fh.readlines():
        if line == "":
            continue

        signals, output = line.strip().split(" | ")

        wrong: Dict[str, int] = {}
        i = 0

        # lazy, but effective approach: iterate through all possible character combinations
        # per signal (the search space for 7 characters is only 7! = 5040 which does not
        # dampen performance too much)
        while True:
            c = ''.join(random.sample("abcdefg", 7))  # drawn configuration
            if c in wrong.keys():
                continue

            # key trick of why this works: every number MUST appear as a signal
            # so there's no noise to filter through
            # use my own ordering to generate the seven segment numbers
            # all orderings will work
            digits = {
                ''.join(sorted([c[0], c[1], c[2], c[4], c[5], c[6]])): 0,
                ''.join(sorted([c[1], c[4]])): 1,
                ''.join(sorted([c[0], c[1], c[3], c[5], c[6]])): 2,
                ''.join(sorted([c[0], c[1], c[3], c[4], c[6]])): 3,
                ''.join(sorted([c[1], c[2], c[3], c[4]])): 4,
                ''.join(sorted([c[0], c[2], c[3], c[4], c[6]])): 5,
                ''.join(sorted([c[0], c[2], c[3], c[4], c[5], c[6]])): 6,
                ''.join(sorted([c[0], c[1], c[4]])): 7,
                ''.join(sorted([c[0], c[1], c[2], c[3], c[4], c[5], c[6]])): 8,
                ''.join(sorted([c[0], c[1], c[2], c[3], c[4], c[6]])): 9
            }

            # check whether all signals appear in the configuration
            correct = all(map(lambda s: ''.join(s) in digits.keys(), map(lambda s: sorted(s), filter(lambda s: s != "", signals.split(" ")))))

            if correct or i == 5039:  # maximum search space (7! - 1)
                break

            wrong[c] = True
            i = i + 1

        if not correct:
            print("No configuration within the search space could be found")
            sys.exit(1)

        # translate each output into concated numbers (abcd, abbb, abcd, abcd => 5, 3, 5, 5 => 5355)
        outputs.append(int(''.join([str(x) for x in [*map(lambda s: digits[''.join(s)], map(lambda s: sorted(s), filter(lambda s: s != "", output.split(" "))))]])))

    print("[part two] the total sum of all decoded outputs is", sum(outputs))
