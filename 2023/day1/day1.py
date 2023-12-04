def run(filename, part2=False):
    with open(filename) as f:
        lines = f.read().splitlines()

    nums = []
    for line in lines:
        if part2:
            words = [
                ("one", "1"),
                ("two", "2"),
                ("three", "3"),
                ("four", "4"),
                ("five", "5"),
                ("six", "6"),
                ("seven", "7"),
                ("eight", "8"),
                ("nine", "9"),
            ]
            for w in words:
                line = line.replace(w[0], f"{w[0]}{w[1]}{w[0]}")
        for c in line:
            if c.isdigit():
                first = int(c)
                break
        for c in reversed(line):
            if c.isdigit():
                second = int(c)
                break
        nums.append(first * 10 + second)
    return sum(nums)


assert run("test.txt") == 142
print(run("input.txt"))

assert run("test2.txt", True) == 281
print(run("input.txt", True))
