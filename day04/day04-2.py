#!/usr/bin/python3

"""
I hate these kinds of grid puzzles so much, I had to do it in Python
because they are even worse in Rust. I even had to make a separate
grid to visualize and compare to the test input.
"""


def rotate(map):
    n = len(map)
    res = [["."] * n for _ in range(n)]
    for i in range(n):
        for j in range(n):
            res[j][n - i - 1] = map[i][j]

    return res


def main():
    with open("day04-input.txt", "r") as file:
        input = file.read()
    map = input.split("\n")
    map = map[:-1]
    VIS = [["."] * len(map) for _ in range(len(map))]
    result = 0
    result = find(result, map, VIS)
    map = rotate(map)
    VIS = rotate(VIS)
    result = find(result, map, VIS)
    map = rotate(map)
    VIS = rotate(VIS)
    result = find(result, map, VIS)
    map = rotate(map)
    VIS = rotate(VIS)
    result = find(result, map, VIS)
    VIS = rotate(VIS)
    # for line in VIS:
    #    print("".join(line))
    print(result)


def find(result, map, VIS):
    for lineindex, line in enumerate(map):
        for index, char in enumerate(line):
            try:
                if char == "A":
                    posx = index
                    posy = lineindex
                    if posx == 0:
                        continue
                    if posy == 0:
                        continue
                    if (
                        (map[posy - 1][posx - 1] == "M")
                        & (map[posy - 1][posx + 1] == "M")
                        & (map[posy + 1][posx - 1] == "S")
                        & (map[posy + 1][posx + 1] == "S")
                    ):
                        result += 1
                        VIS[posy][posx] = "A"
                        VIS[posy - 1][posx - 1] = "M"
                        VIS[posy - 1][posx + 1] = "M"
                        VIS[posy + 1][posx - 1] = "S"
                        VIS[posy + 1][posx + 1] = "S"
            except IndexError:
                continue
    return result


if __name__ == main():
    main()
