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
    chars = ["X", "M", "A", "S"]
    for index, line in enumerate(map):
        str = "".join(line)
        num = str.count("XMAS")
        result += num

    for lineindex, line in enumerate(map):
        for index, char in enumerate(line):
            try:
                if char == "X":
                    posx = index
                    posy = lineindex
                    tmpres = 0
                    for i in [1, 2, 3]:
                        if map[posy + i][posx + i] == chars[i]:
                            tmpres += 1
                        else:
                            break
                    if tmpres == 3:
                        result += 1
                        VIS[posy][posx] = "X"
                        VIS[posy + 1][posx + 1] = "M"
                        VIS[posy + 2][posx + 2] = "A"
                        VIS[posy + 3][posx + 3] = "S"
            except IndexError:
                continue
    return result


if __name__ == main():
    main()
