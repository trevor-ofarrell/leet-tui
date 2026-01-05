def checkInclusion(s1, s2):
    if len(s1) > len(s2):
        return False
    count = [0] * 26
    for i in range(len(s1)):
        count[ord(s1[i]) - 97] += 1
        count[ord(s2[i]) - 97] -= 1
    if all(c == 0 for c in count):
        return True
    for i in range(len(s1), len(s2)):
        count[ord(s2[i]) - 97] -= 1
        count[ord(s2[i - len(s1)]) - 97] += 1
        if all(c == 0 for c in count):
            return True
    return False
