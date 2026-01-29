def find_outlier(ints):
    even_count = 0
    for i in ints[:3]:
        print(i)
        if i % 2 == 0:
            even_count += 1
    return [i for i in ints if i % 2 != 0][0] if even_count > 1 else [i for i in ints if i % 2 == 0][0]
