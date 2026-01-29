def sum_two_smallest_numbers(a):
    a.sort()
    return a[0] + a[1]


def sum_two_sm_nums(a):
    return sum(sorted(a)[:2])
