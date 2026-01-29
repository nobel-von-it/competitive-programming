def alphabet_position(text):
    alphabet = "abcdefghijklmnopqrstuvwxyz"
    res = []
    for i in text.lower():
        if i in alphabet:
            res.append(alphabet.index(i)+1)
    return " ".join(str(i) for i in res)


def alph_position(text):
    return " ".join(str(ord(i)) for i in text.lower() if i.isalpha())
