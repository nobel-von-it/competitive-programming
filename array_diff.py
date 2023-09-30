def array_diff(a, b):
    res = []
    for i in a:
        if i not in b:
            res.append(i)
    return res


def array_diff2(a, b):
    return [x for x in a if x not in b]