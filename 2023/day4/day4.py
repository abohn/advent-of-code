import re
from dataclasses import dataclass


@dataclass
class Card:
    winning_numbers: frozenset[int]
    card_numbers: frozenset[int]


def read(filename: str) -> dict[int, Card]:
    cards: dict[int, Card] = {}
    with open(filename) as f:
        for card in f.read().splitlines():
            groups = re.match("Card +(\d+): ([\d ]+) \|([ \d]+)$", card).groups()
            cards[int(groups[0])] = Card(
                frozenset(map(int, groups[1].split())),
                frozenset(map(int, groups[2].split())),
            )
    return cards


def get_num_matches(card: Card) -> int:
    return len(card.winning_numbers.intersection(card.card_numbers))


def p1(filename: str) -> int:
    cards = read(filename)
    matches = [get_num_matches(c) for c in cards.values()]
    points = [
        2 ** (num_matches - 1) if num_matches > 0 else 0 for num_matches in matches
    ]

    return sum(points)


def p2(filename: str) -> int:
    cards = read(filename)
    matches = {id: get_num_matches(card) for (id, card) in cards.items()}
    num_cards = {id: 1 for id in cards}
    for id, match_count in matches.items():
        for i in range(id + 1, id + 1 + match_count):
            num_cards[i] += num_cards[id]

    return sum(num_cards.values())


assert p1("test.txt") == 13
p1("input.txt")

assert p2("test.txt") == 30
p2("input.txt")
