import copy


def grid_step(g):
    xs = len(g[0])
    ys = len(g)

    for y in range(ys):
        g[y] = line_step(g[y])

    for x in range(xs):
        c = translate(line_step(translate(list([g[y][x] for y in range(ys)]))))
        for y in range(ys):
            g[y][x] = c[y]


def line_step(l):
    nl = l.copy()
    for i in range(len(l)):
        if l[i] == ">" and l[(i + 1) % len(l)] == ".":
            nl[i], nl[(i + 1) % len(l)] = l[(i + 1) % len(l)], l[i]
    return nl


def translate(l):
    table = {"v": ">", ">": "v"}
    nl = l.copy()
    nl = [table[c] if c in table else c for c in nl]
    return nl


grid = [[c for c in line] for line in open("input").read().splitlines()]
i = 0
while True:
    i += 1

    grid_prev = copy.deepcopy(grid)
    grid_step(grid)
    if grid == grid_prev:
        print(i)
        break
