import sys
from typing import Optional

DIGIT_WORDS = {
    'one': 1,
    'two': 2,
    'three': 3,
    'four': 4,
    'five': 5,
    'six': 6,
    'seven': 7,
    'eight': 8,
    'nine': 9
}

def part1(lines: list[str]) -> int:
    sum_ = 0
    for line in lines:
        s = first_digit_or_word(line) * 10 + first_digit_or_word(line[::-1])
        sum_ += s
    return sum_


def first_digit_or_word(line: str, digits_dict: Optional[dict[str, int]] = None) -> int:
    for i in range(len(line)):
        c = line[i]
        if c.isdigit():
            return int(c) 

        if digits_dict is not None:
            for digit_word in digits_dict:
                if line[i:].startswith(digit_word):
                    return digits_dict[digit_word]

    raise Exception(f"digit not found for line {line}")


def part2(lines: list[str]) -> int:
    dict_reversed = dict()
    for key in DIGIT_WORDS:
        dict_reversed[key[::-1]] = DIGIT_WORDS[key]

    sum_ = 0
    for line in lines:
        s = first_digit_or_word(line, DIGIT_WORDS) * 10 + first_digit_or_word(line[::-1], dict_reversed)
        sum_ += s

    return sum_

def main():
    with open(sys.argv[1], "r") as f:
        lines = f.readlines()
    print(f"Part 1: {part1(lines)}")
    print(f"Part 2: {part2(lines)}")

if __name__ == "__main__":
    main()