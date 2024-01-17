from itertools import product


class DeterministicDice:
    def __init__(self) -> None:
        self.rolls = 0

    def roll(self) -> int:
        self.rolls += 1
        return (self.rolls - 1) % 100 + 1


def wrap(n: int) -> int:
    return ((n - 1) % 10) + 1


players = [8, 2]
scores = [0, 0]
die = DeterministicDice()
j = 0
while True:
    i = j % len(players)

    rolls = sum(die.roll() for _ in range(3))
    players[i] = wrap(players[i] + rolls)
    scores[i] += players[i]

    if max(scores) >= 1000:
        break

    j += 1

print(min(scores) * die.rolls)


cache = {}


def wins(players, scores):
    key = players + scores
    if key in cache:
        return cache[key]

    r = [0, 0]

    for s in (sum(s) for s in product([1, 2, 3], repeat=3)):
        p0 = wrap(players[0] + s)
        s0 = scores[0] + p0
        if s0 >= 21:
            r[0] += 1
        else:
            x, y = wins((players[1], p0), (scores[1], s0))
            r[0] += y
            r[1] += x

    cache[key] = r
    return r


print(wins((8, 2), (0, 0)))
