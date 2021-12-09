#!/usr/bin/env python3

import os
import sys
from functools import reduce
from typing import List, Tuple


def main(lines: List[str]):
    cols: int = len(lines[0])
    rows: int = len(lines)
    height_map: List[int] = [
        *map(lambda s: int(s), reduce(lambda acc, curr: str(acc) + curr, lines, ""))]

    print("=== [ part one ] ==================================")
    print("|| get the low points ||")
    print()
    lowpoints, risk_level = part_one(height_map, rows, cols)

    print(f"n_low points: \t{len(lowpoints)}")
    print(f"risk level: \t{risk_level}")
    print()

    print("=== [ part two ] ==================================")
    print("|| region grow the basins ||")
    print()
    basins = part_two(height_map, lowpoints, rows, cols)
    biggest_3 = sorted(basins, key=lambda basin: len(basin), reverse=True)[:3]

    print(f"n_basins: \t\t{len(basins)}")
    print(f"size top3 basins: \t{reduce(lambda acc, curr: acc * len(curr), biggest_3, 1)}")


def part_one(height_map: List[int], rows: int, cols: int) -> Tuple[List[int], int]:
    """
        part_one returns the low points for a given height map with dimension (rows, cols)
    """

    lowpoints: List[int] = []
    for pos in range(len(height_map)):
        height: int = height_map[pos]

        # create a N4 neighborhood and filter out the current position, if existing
        neighbors = get_n4_neighborhood(pos, rows, cols)

        is_lowpoint = [*map(lambda neighbor: height_map[neighbor] > height, neighbors)]
        if all(is_lowpoint):
            lowpoints.append(pos)

    risk_level: int = sum([*map(lambda idx: height_map[idx] + 1, lowpoints)])
    return lowpoints, risk_level


def part_two(height_map: List[int], lowpoints: List[int], rows: int, cols: int) -> List[List[int]]:
    """
        part_two determines the basins of each given low point on a height map with
        dimension (rows, cols) by conducting region growth in an N4 neighborhood
    """

    basins: List[List] = []
    for lowpoint in lowpoints:
        basin: List[int] = []

        def grow_basin(p: int):
            basin.append(p)
            neighbors: List[int] = get_n4_neighborhood(p, rows, cols)
            neighbors = [*filter(lambda neighbor: height_map[neighbor] != 9, neighbors)]

            for neighbor in neighbors:
                if neighbor in basin:  # ignore already existing points
                    continue

                grow_basin(neighbor)

        grow_basin(lowpoint)
        if len(basin) > 0:
            basins.append(basin)

    return basins


def within_limit(x: int, n: int) -> int:
    return 0 if x < 0 else n - 1 if x >= n else x


def get_n4_neighborhood(pos: int, rows: int, cols: int) -> List[int]:
    row: int = pos // cols
    col: int = pos % cols

    neighbors: List[Tuple[int, int]] = [
        (within_limit(row - 1, rows), within_limit(col, cols)),
        (within_limit(row, rows), within_limit(col + 1, cols)),
        (within_limit(row + 1, rows), within_limit(col, cols)),
        (within_limit(row, rows), within_limit(col - 1, cols))
    ]

    # translate back from xy to sequential number
    translated: List[int] = [*map(lambda neighbor: neighbor[0] * cols + neighbor[1], neighbors)]
    return [*filter(lambda neighbor: neighbor != pos, translated)]


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
