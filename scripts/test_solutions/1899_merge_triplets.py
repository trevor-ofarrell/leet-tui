def mergeTriplets(triplets, target):
    good = set()
    for a, b, c in triplets:
        if a <= target[0] and b <= target[1] and c <= target[2]:
            if a == target[0]:
                good.add(0)
            if b == target[1]:
                good.add(1)
            if c == target[2]:
                good.add(2)
    return len(good) == 3
