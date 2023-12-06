from math import sqrt, floor, ceil
import sys

def part1(time_distance: list[tuple[int, int]]) -> int:
    res = 1
    for time, distance in time_distance:
        res *= part2(time, distance)
    return res


def roots(time, distance) -> int:
    # assuming discriminant is > 0
    d = sqrt(time * time - 4 * distance)
    return (time - d) / 2, (time + d) / 2


def part2(time, distance) -> int:
    x1, x2 = roots(time, distance)
    res =  min([floor(x2), time]) - max([ceil(x1), 0]) + 1
    if x1 == ceil(x1):
        res -= 1
    if x2 == floor(x2):
        res -= 1
    return res
    

def parse_single_number(line: str) -> int:
    return int(line.split(":")[1].replace(" ", "").strip())


def main():
    with open(sys.argv[1], "r") as f:
        [time, distance] = f.readlines()
        time_distance = list(map(lambda t_d: (int(t_d[0]), int(t_d[1])), list(zip(time.split(), distance.split()))[1:]))
        print(f"Part 1: {part1(time_distance)}")
        time_ = parse_single_number(time)
        distance_ = parse_single_number(distance)
        print(f"Part 2: {part2(time_, distance_)}")


if __name__ == "__main__":
    main()