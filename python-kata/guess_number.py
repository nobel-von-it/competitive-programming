def guess_my_number(guess, number="123-451-2345"):
    res = ""
    for i in number:
        if i not in guess and not i == "-":
            res += "#"
        else:
            res += i
    return res
