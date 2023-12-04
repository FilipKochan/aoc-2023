from collections import defaultdict
import sys

MAX_CARDS = defaultdict(lambda:float("inf"))
MAX_CARDS['red'] = 12
MAX_CARDS['green'] = 13
MAX_CARDS['blue'] = 14

def part2(lines: list[str]) -> int:
    powers = 0
    for line in lines:
        [_, rest] = line.split(':')
        replaced = rest.replace(";", ",")

        min_amounts = defaultdict(int)
        for draw in replaced.split(','):
            amount, color = draw.split()
            color = color.strip()
            amount = amount.strip()
            min_amounts[color] = max(min_amounts[color], int(amount))
        
        powers += min_amounts['red'] * min_amounts['green'] * min_amounts['blue']
        
    return powers


def part1(lines: list[str]) -> int:
    ids_sum = 0
    for line in lines:
        [game, rest] = line.split(':')
        game_id = int(game.split()[-1])
        replaced = rest.replace(";", ",")
        
        ok = True
        for draw in replaced.split(','):
            amount, color = draw.split()
            color = color.strip()
            if int(amount.strip()) > MAX_CARDS[color]:
                ok = False
                break
                
        if ok:
            ids_sum += game_id

    return ids_sum

def main():
    with open(sys.argv[1]) as f:
        lines = f.readlines()
        print(f"Part 1: {part1(lines)}")
        print(f"Part 2: {part2(lines)}")

if __name__ == "__main__":
    main()