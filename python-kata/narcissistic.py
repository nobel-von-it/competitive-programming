def narcissistic(n):
    s = str(n)
    sum = 0
    for i in s:
        sum += int(i) ** len(s)
    return True if sum == n else False
