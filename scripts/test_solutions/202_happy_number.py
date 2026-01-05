def isHappy(n):
    seen = set()
    while n != 1 and n not in seen:
        seen.add(n)
        total = 0
        while n > 0:
            total += (n % 10) ** 2
            n //= 10
        n = total
    return n == 1
