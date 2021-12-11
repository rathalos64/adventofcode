#!/usr/bin/env python3

import os
import sys
from functools import reduce
from typing import List, Tuple, Generator


def main(lines: List[str]):
    cols: int = len(lines[0])
    rows: int = len(lines)
    energy_map: List[int] = [
        *map(lambda s: int(s), reduce(lambda acc, curr: str(acc) + curr, lines, ""))]

    print("=== [ part one ] ============================")
    print("|| how many octo flashes after 100 steps? ||")

    step = 0
    for flashes, _ in octo_party(energy_map.copy(), rows, cols, 100):
        step += 1

    print(f"After step {step}: {flashes} flashes")

    print()
    print("=== [ part two ] ============================")
    print("|| how long until complete synchronization? ||")
    step = 0
    for flashes, energy_map in octo_party(energy_map.copy(), rows, cols, 10000):
        step += 1
        if energy_map.count(energy_map[0]) == len(energy_map):
            break

    print(f"1st total synchronization reached after {step} steps: {flashes} flashes")


def octo_party(energy_map: List[int], rows: int, cols: int, steps: int) -> Generator[Tuple[int, List[int]], None, None]:
    """
        octo_party simulates totally cute bioluminescent dumbo octopuses
        that are arrange in a given energy_map with dimension (rows, cols)
        for n given steps. Acting as a generator on every step, it returns
        the current energy level of every octocutie along with the number of flashes.
    """
    octopusses: int = len(energy_map)

    flashes: int = 0
    for step in range(steps):
        for octopus in range(octopusses):
            energy_map[octopus] += 1

        # handle flashes
        flashing: List[int] = [*filter(lambda octopus: energy_map[octopus] > 9, range(octopusses))]
        for idx in flashing:
            energy_map[idx] = 0  # reset the cutie
            flashes += 1

            for neighbor in get_n8_neighborhood(idx, rows, cols):
                if neighbor in flashing:
                    continue

                energy_map[neighbor] += 1
                if energy_map[neighbor] > 9:
                    flashing.append(neighbor)

        yield flashes, energy_map


def within_limit(x: int, n: int) -> int:
    return 0 if x < 0 else n - 1 if x >= n else x


def get_n8_neighborhood(pos: int, rows: int, cols: int) -> List[int]:
    row: int = pos // cols
    col: int = pos % cols

    neighbors: List[Tuple[int, int]] = [
        (within_limit(row - 1, rows), within_limit(col, cols)),
        (within_limit(row - 1, rows), within_limit(col + 1, cols)),
        (within_limit(row, rows), within_limit(col + 1, cols)),
        (within_limit(row + 1, rows), within_limit(col + 1, cols)),
        (within_limit(row + 1, rows), within_limit(col, cols)),
        (within_limit(row + 1, rows), within_limit(col - 1, cols)),
        (within_limit(row, rows), within_limit(col - 1, cols)),
        (within_limit(row - 1, rows), within_limit(col - 1, cols)),
    ]

    # translate back from xy to sequential number
    translated: List[int] = [*map(lambda neighbor: neighbor[0] * cols + neighbor[1], neighbors)]
    return list(set([*filter(lambda neighbor: neighbor != pos, translated)]))


if __name__ == "__main__":
    path: str = sys.argv[1] if len(sys.argv) > 1 else ""
    if not os.path.exists(path):
        print(f"given path {path} does not exist")
        sys.exit(1)

    with open(path) as fh:
        lines: List[str] = [*map(lambda line: line.strip(),
                                 filter(lambda line: line != "", fh.readlines()))]
        if len(lines) == 0:
            print("no lines detected")
            sys.exit(1)

    main(lines)
