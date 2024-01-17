from __future__ import annotations


def overlap_segment(x1: int, x2: int, y1: int, y2: int) -> tuple[int, int] | None:
    offset = max(0, min(x2, y2) - max(x1, y1))
    origin = sorted([x1, x2, y1, y2])[1]
    if offset > 0:
        return origin, origin + offset
    else:
        return None


class Cuboid:
    def __init__(
        self, x1: int, x2: int, y1: int, y2: int, z1: int, z2: int, on: bool = False
    ) -> None:
        self.x1 = x1
        self.x2 = x2
        self.y1 = y1
        self.y2 = y2
        self.z1 = z1
        self.z2 = z2
        self.on = on

    def is_inside(self, other: Cuboid) -> bool:
        return (
            other.x1 <= self.x1 <= other.x2
            and other.x1 <= self.x2 <= self.x2
            and other.y1 <= self.y1 <= other.y2
            and other.y1 <= self.y2 <= self.y2
            and other.z1 <= self.z1 <= other.z2
            and other.z1 <= self.z2 <= self.z2
        )

    def has_point(self, point: tuple[int, int, int]) -> bool:
        return (
            self.x1 <= point[0] <= self.x2
            and self.y1 <= point[1] <= self.y2
            and self.z1 <= point[2] <= self.z2
        )

    def volume(self) -> int:
        return (
            (self.x2 - self.x1 + 1) * (self.y2 - self.y1 + 1) * (self.z2 - self.z1 + 1)
        )

    def overlap(self, other: Cuboid) -> Cuboid | None:
        x = overlap_segment(self.x1, self.x2, other.x1, other.x2)
        y = overlap_segment(self.y1, self.y2, other.y1, other.y2)
        z = overlap_segment(self.z1, self.z2, other.z1, other.z2)

        if x is None or y is None or z is None:
            return None

        return Cuboid(x[0], x[1], y[0], y[1], z[0], z[1])

    def __str__(self) -> str:
        return f"{'on' if self.on else 'off'} x={self.x1}..{self.x2},y={self.y1}..{self.y2},z={self.z1}..{self.z2}"


def parse_input(data: str) -> list[Cuboid]:
    lines = data.splitlines()

    cuboids = []

    for line in lines:
        parts = line.split()

        on = parts[0] == "on"
        coords = parts[1].split(",")

        x1, x2 = [int(x) for x in coords[0][2:].split("..")]
        y1, y2 = [int(y) for y in coords[1][2:].split("..")]
        z1, z2 = [int(z) for z in coords[2][2:].split("..")]

        cuboids.append(Cuboid(x1, x2, y1, y2, z1, z2, on))

    return cuboids


def part1(data: str) -> int:
    input = parse_input(data)
    input = [box for box in input if box.is_inside(Cuboid(-50, 50, -50, 50, -50, 50))]

    grid = {}

    for p in (
        (x, y, z)
        for x in range(-50, 51)
        for y in range(-50, 51)
        for z in range(-50, 51)
    ):
        for box in input:
            if box.has_point(p):
                grid[p] = True if box.on else False

    return sum(grid.values())


def part2(data: str) -> int:
    input = parse_input(data)

    cuboids = []

    for i in input:
        if i.on:
            cuboids.append(i)

        c = []
        for j in cuboids:
            if j is not i:
                o = j.overlap(i)
                if o:
                    if not j.on:
                        o.on = True
                    c.append(o)
        cuboids += c

    return sum([box.volume() if box.on else -box.volume() for box in cuboids])


data = open("input").read()
print(part1(data))
print(part2(data))
