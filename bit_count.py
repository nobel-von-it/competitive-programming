def count_bits(n):
    s = ''
    while n > 0:
        s += str(n % 2)
        n //= 2
    return s.count('1')


def count_bits2(n):
    return bin(n).count('1')
