from dataclasses import dataclass
import re
import math
from typing import NamedTuple, List, DefaultDict
from collections import defaultdict


def read(filename: str) -> list[str]:
    with open(filename) as f:
        lines = f.read().splitlines()
    return lines


@dataclass
class SchematicNumber:
    row: int
    col_span: tuple[int, int]
    number: int


class Coord(NamedTuple):
    row: int
    col: int


def find_numbers(schematic: list[str]) -> list[SchematicNumber]:
    return [
        SchematicNumber(row, match.span(), int(match[0]))
        for row, data in enumerate(schematic)
        for match in re.finditer(r"(\d+)", data)
    ]


def is_part_number(num: SchematicNumber, schematic: list[str]) -> bool:
    # make a box surrounding the number, handling the edges.
    # todo: consider adding a bounding set of periods to make edges easier
    start_col = max(0, num.col_span[0] - 1)
    end_col = min(len(schematic[num.row]) - 1, num.col_span[1])
    start_row = max(num.row - 1, 0)
    end_row = min(num.row + 1, len(schematic) - 1)

    matches = [
        re.search(r"[^\d.]", schematic[row][start_col : end_col + 1]) is not None
        for row in range(start_row, end_row + 1)
    ]
    return any(matches)


def nearby_stars(num: SchematicNumber, schematic: list[str]) -> list[Coord]:
    # make a box surrounding the number, handling the edges.
    start_col = max(0, num.col_span[0] - 1)
    end_col = min(len(schematic[num.row]) - 1, num.col_span[1])
    start_row = max(num.row - 1, 0)
    end_row = min(num.row + 1, len(schematic) - 1)

    stars = [
        # column indexing is off by start_col due to slicing
        Coord(row, start_col + match.start())
        for row in range(start_row, end_row + 1)
        for match in re.finditer(r"\*", schematic[row][start_col : end_col + 1])
    ]
    return stars


def part1(filename: str) -> int:
    schematic = read(filename)
    nums = find_numbers(schematic)
    part_nums = [num.number if is_part_number(num, schematic) else 0 for num in nums]
    return sum(part_nums)


def get_star_neighbors(
    nums: list[SchematicNumber], schematic: list[str]
) -> list[list[int]]:
    """Returns a list of stars, where a star is a list of numbers surrounding each asterisk"""
    star_neighbors: DefaultDict[Coord, List[int]] = defaultdict(list[int])
    for num in nums:
        for star in nearby_stars(num, schematic):
            star_neighbors[star].append(num.number)

    return list(star_neighbors.values())


def part2(filename: str) -> int:
    schematic = read(filename)
    nums = find_numbers(schematic)
    star_neighbors = get_star_neighbors(nums, schematic)
    gear_ratios = [
        math.prod(neighbors) if len(neighbors) == 2 else 0
        for neighbors in star_neighbors
    ]
    return sum(gear_ratios)


assert part1("test.txt") == 4361
print(part1("input.txt"))

assert part2("test.txt") == 467835
part2("input.txt")
