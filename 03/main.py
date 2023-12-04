import sys

def is_symbol(ch: str) -> bool:
    return ch != '.' and ch != '\n' and not ch.isdigit()

def has_symbol(lines: list[str], at: tuple[int, int]) -> bool:
    row, col = at

    for val in range(9):
        o_r = val // 3 - 1
        o_c = val % 3 - 1

        if o_r == 0 and o_c == 0:
            continue

        r = o_r + row
        c = o_c + col

        try:
            ch = lines[r][c]
            if is_symbol(ch):
                return True 
        except IndexError:
            pass

    return False

def part1(lines: list[str]) -> int:
    sum_ = 0
    for row, line in enumerate(lines):
        num = 0
        num_has_symbol = False

        for col, char in enumerate(line):
            if char.isdigit():
                num *= 10
                num += int(char)
                if has_symbol(lines, (row, col)):
                    num_has_symbol = True
            else:
                if num_has_symbol and num > 0:
                    sum_ += num
                num = 0
                num_has_symbol = False
        
        if num_has_symbol:
            sum_ += num
        
    return sum_

def part2(lines: list[str]) -> int:
    num_values: dict[int, int] = dict()
    num_references: dict[tuple[int, int], int] = dict()
    num_id = 1

    for row, line in enumerate(lines):
        num = 0

        for col, char in enumerate(line):
            if char.isdigit():
                num *= 10
                num += int(char)
                num_references[(row, col)] = num_id

            else:
                if num > 0:
                    num_values[num_id] = num
                
                num_id += 1
                num = 0
        
        if num > 0:
            num_values[num_id] = num
            num_id += 1
    
    return compute_grears(lines, num_references, num_values)

def compute_grears(lines: list[str], num_references: dict[tuple[int, int], int], num_values: dict[int, int]) -> int:
    res = 0
    for row, line in enumerate(lines):
            for col, char in enumerate(line):
                adjacent_nums: set[int] = set()
                if char == '*':
                     for val in range(9):
                        o_r = val // 3 - 1
                        o_c = val % 3 - 1

                        if o_r == 0 and o_c == 0:
                            continue

                        r = o_r + row
                        c = o_c + col
                        coord = (r, c)
                        if coord in num_references:
                            adjacent_nums.add(num_references[coord])

                if len(adjacent_nums) == 2:
                    num1, num2 = adjacent_nums
                    res += num_values[num1] * num_values[num2]

    return res          


def main():
    with open(sys.argv[1], "r") as f:
        lines = f.readlines()
        print(f"Part 1: {part1(lines)}")
        print(f"Part 2: {part2(lines)}")
if __name__ == "__main__":
    main()