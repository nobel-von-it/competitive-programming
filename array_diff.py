def array_diff(a, b):
    for i in a:
        if i in b:
            a = a.remove(i)
    return a
