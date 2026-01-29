def is_isogram(s):
    a = ''
    for i in s.lower():
        if i in a:
            return False
        a += i
    return True
