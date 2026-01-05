def countSubstrings(s):
    count = [0]

    def expand(l, r):
        while l >= 0 and r < len(s) and s[l] == s[r]:
            count[0] += 1
            l -= 1
            r += 1

    for i in range(len(s)):
        expand(i, i)
        expand(i, i + 1)
    return count[0]
