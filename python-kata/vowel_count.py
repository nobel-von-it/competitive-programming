def get_count(s):
    v = 'aeiou'
    k = 0
    for i in s:
        if i in v:
            k += 1
    return k