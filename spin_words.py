def spin_words(s):
    a = s.split(" ")
    for i in range(len(a)):
        if len(a[i]) > 4:
            a[i] = a[i][::-1]
    return " ".join(a)
