import functools


lines = open("input").read().splitlines()
ops = [line.split() for line in lines]


def get(values, var):
    return values[var] if var in values else int(var)


@functools.cache
def search(op_index, w, x, y, z):
    if z > 9 ** 6:
        return (False, "")

    if op_index >= len(ops):
        return (z == 0, "")

    values = {"w": w, "x": x, "y": y, "z": z}

    op = ops[op_index]
    if op[0] == "inp":
        for d in range(1, 10):
            values[op[1]] = d
            result = search(op_index + 1, *values.values())
            if result[0]:
                return (True, str(d) + result[1])
        return (False, "")
    elif op[0] == "add":
        values[op[1]] += get(values, op[2])
    elif op[0] == "mul":
        values[op[1]] *= get(values, op[2])
    elif op[0] == "div":
        values[op[1]] //= get(values, op[2])
    elif op[0] == "mod":
        values[op[1]] %= get(values, op[2])
    elif op[0] == "eql":
        values[op[1]] = 1 if values[op[1]] == get(values, op[2]) else 0
    else:
        assert False

    return search(op_index + 1, *values.values())


print(search(0, 0, 0, 0, 0))
