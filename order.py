def order(s):
    a = s.split(" ")
    dig = "123456789"
    res = []
    for i in dig:
        for j in a:
            if i in j:
                res.append(j)
    return " ".join(res)
