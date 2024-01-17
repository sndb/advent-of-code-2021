from collections import defaultdict
from itertools import combinations


def rotate_x(v):
    return (v[0], v[2], -v[1])


def rotate_y(v):
    return (v[2], v[1], -v[0])


def rotate_z(v):
    return (v[1], -v[0], v[2])


def manhattan(x, y):
    return abs(x[0] - y[0]) + abs(x[1] - y[1]) + abs(x[2] - y[2])


def sub(x, y):
    return (x[0] - y[0], x[1] - y[1], x[2] - y[2])


def add(x, y):
    return (x[0] + y[0], x[1] + y[1], x[2] + y[2])


def rotations(v):
    rots = {}
    k = (1, 2, 3)
    for _ in range(4):
        v, k = rotate_x(v), rotate_x(k)
        for _ in range(4):
            v, k = rotate_y(v), rotate_y(k)
            for _ in range(4):
                v, k = rotate_z(v), rotate_z(k)
                rots[k] = v
    return rots.values()


def overlap(vecs1, vecs2):
    for rotated in zip(*[rotations(v) for v in vecs2]):
        for v1 in vecs1:
            for v2 in rotated:
                offset = sub(v2, v1)
                intersection = set(vecs1) & {sub(v, offset) for v in rotated}
                if len(intersection) >= 12:
                    return offset, rotated
    return None, None


scanners = defaultdict(list)


def merge(vecs1, vecs2):
    offset, rotated = overlap(vecs1, vecs2)
    if not rotated:
        return None

    merged = tuple(set(vecs1) | {sub(v, offset) for v in rotated})

    if vecs1 in scanners:
        scanners[merged] = scanners[vecs1]
        del scanners[vecs1]
    if vecs2 in scanners:
        scanners[merged] += [add(v, offset) for v in scanners[vecs2]]
        del scanners[vecs2]
    scanners[merged].append(offset)

    return merged


def cycle(input):
    for x, y in combinations(input, 2):
        m = merge(tuple(x), tuple(y))
        if m:
            input.remove(x)
            input.remove(y)
            input.insert(0, m)
            return


def solve(input):
    while len(input) > 1:
        print(len(input))
        cycle(input)

    return (
        len(input[0]),
        max(
            [
                manhattan(x, y)
                for x, y in combinations(list(scanners.values())[0] + [(0, 0, 0)], 2)
            ]
        ),
    )


def parse_input(data):
    return [
        [eval(f"({line})") for line in scanner.splitlines()[1:]]
        for scanner in data.split("\n\n")
    ]


print(solve(parse_input(open("input").read())))
