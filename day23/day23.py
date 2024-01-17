from copy import deepcopy


State = tuple[dict[str, list[str]], list[str]]
costs = {"a": 1, "b": 10, "c": 100, "d": 1000}
cache = {}


def solve(state: State) -> int:
    rooms, hallway = state

    key = (tuple((k, tuple(v)) for k, v in rooms.items()), tuple(hallway))
    if key in cache:
        return cache[key]

    if done(state):
        return 0

    for position, amphipod in enumerate(hallway):
        if (
            amphipod in rooms
            and can_move_to(amphipod, rooms[amphipod])
            and clear_path(amphipod, position, hallway)
        ):
            dest = destination_index(rooms[amphipod])
            assert dest is not None
            distance = dest + 1 + abs(room_index(amphipod) - position)
            cost = costs[amphipod] * distance

            hallway[position] = "."
            new_hallway = hallway.copy()

            new_rooms = deepcopy(rooms)
            new_rooms[amphipod][dest] = amphipod

            return cost + solve((new_rooms, new_hallway))

    answer = int(1e9)
    for k, room in rooms.items():
        if not can_move_from(k, room):
            continue

        origin = origin_index(room)
        if origin is None:
            continue

        amphipod = room[origin]

        for to in range(len(hallway)):
            if to in [2, 4, 6, 8]:
                continue

            if hallway[to] != ".":
                continue

            if clear_path(k, to, hallway):
                distance = origin + 1 + abs(to - room_index(k))

                new_hallway = hallway.copy()
                new_hallway[to] = amphipod

                new_rooms = deepcopy(rooms)
                new_rooms[k][origin] = "."

                answer = min(
                    answer,
                    costs[amphipod] * distance + solve((new_rooms, new_hallway)),
                )

    cache[key] = answer
    return answer


def done(state: State) -> bool:
    rooms, _ = state
    for k, room in rooms.items():
        for amphipod in room:
            if amphipod != k:
                return False
    return True


def can_move_from(amphipod: str, room: list[str]) -> bool:
    for spot in room:
        if spot != amphipod and spot != ".":
            return True
    return False


def can_move_to(amphipod: str, room: list[str]) -> bool:
    for spot in room:
        if spot != amphipod and spot != ".":
            return False
    return True


def room_index(amphipod: str) -> int:
    return {"a": 2, "b": 4, "c": 6, "d": 8}[amphipod]


def origin_index(room: list[str]) -> int | None:
    for i, spot in enumerate(room):
        if spot != ".":
            return i
    return None


def destination_index(room: list[str]) -> int | None:
    for i, spot in reversed(list(enumerate(room))):
        if spot == ".":
            return i
    return None


def clear_path(amphipod: str, position: int, hallway: list[str]) -> bool:
    ai = room_index(amphipod)
    for hi in range(len(hallway)):
        if (ai < hi < position or position < hi < ai) and hallway[hi] != ".":
            return False
    return True


def parse_input(input: str) -> State:
    data = [c.lower() for line in input.splitlines() for c in line if c in "ABCD"]
    rooms = {"abcd"[i]: data[i::4] for i in range(4)}
    hallway = ["."] * 11
    return (rooms, hallway)


print(solve(parse_input(open("input").read())))
