def digital_root(n):
    while n >= 10:
        s = str(n)
        n = 0
        for i in s:
            n += int(i)
    return n
