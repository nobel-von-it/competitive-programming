def find_it(a):
    for i in a:
        tmp = 0
        for j in a:
            if j == i:
                tmp += 1
        if tmp % 2 != 0:
            return i


# or better

def find_it2(a):
    for i in a:
        if a.count(i) % 2 != 0:
            return i
