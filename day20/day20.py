def parse_input(data: str) -> tuple[str, list[str]]:
    input = data.splitlines()
    algo = input[0]
    image = input[2:]
    return algo, image


def decode(image: list[str], row: int, column: int) -> int:
    s = ""
    for i in range(row - 1, row + 2):
        for j in range(column - 1, column + 2):
            try:
                s += image[i][j]
            except IndexError:
                s += "."
    n = 0
    for c in s:
        n <<= 1
        n += 1 if c == "#" else 0
    return n


def decorate(image: list[str]) -> list[str]:
    return (
        ["." * (2 + len(image[0]))]
        + ["." + line + "." for line in image]
        + ["." * (2 + len(image[0]))]
    )


def count(image: list[str]) -> int:
    return len([pixel for line in image for pixel in line if pixel == "#"])


def step(image: list[str], algo: str) -> list[str]:
    for _ in range(3):
        image = decorate(image)
    return [
        "".join([algo[decode(image, row, column)] for column in range(len(image[0]))])
        for row in range(len(image))
    ]


algo, image = parse_input(open("input").read())

for i in range(25):
    image = step(image, algo)
    image = step(image, algo)
    image = [row[4:-4] for row in image[4:-4]]
    if i == 0:
        print(count(image))
print(count(image))
