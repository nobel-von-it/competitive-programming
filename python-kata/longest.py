def longest(a1, a2):
    print("".join(sorted(set(a1).union(set(a2)))))


longest('abcdefghijklmnopqrstuvwxyz', 'abcdefghijklmnopqrstuvwxyz')
