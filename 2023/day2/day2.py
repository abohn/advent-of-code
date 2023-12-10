import parse
import re
import math


def read(filename: str) -> str:
    with open(filename) as f:
        lines = f.read().splitlines()
    return lines


def parse_into_games(lines: list[str]):
    return [parse.parse("Game {id}: {data}", line) for line in lines]


COLORS = ["red", "green", "blue"]


def get_color_maxes(data: str) -> dict[str, int]:
    return {
        color: max(map(int, re.findall(f"(\d+) {color}", data))) for color in COLORS
    }


def part1(filename: str) -> int:
    lines = read(filename)

    proposed_set = {"red": 12, "green": 13, "blue": 14}

    games = parse_into_games(lines)
    total = 0
    for game in games:
        color_maxes = get_color_maxes(game["data"])

        if all(proposed_set[c] >= color_maxes[c] for c in COLORS):
            total += int(game["id"])
    return total


def get_cube_power(data: str) -> int:
    color_maxes = get_color_maxes(data)
    return math.prod(color_maxes.values())


def part2(filename: str) -> int:
    lines = read(filename)
    games = parse_into_games(lines)
    return sum([get_cube_power(game["data"]) for game in games])


assert part1("test.txt") == 8
print(part1("input.txt"))

assert part2("test.txt") == 2286
print(part2("input.txt"))
