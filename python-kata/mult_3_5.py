def solution(n):
    res = 0
    for i in range(n):
        if i % 3 == 0 or i % 5 == 0:
            res += i
    return res


def solution2(n):
    return sum(i for i in range(n) if i % 3 == 0 or i % 5 == 0)
