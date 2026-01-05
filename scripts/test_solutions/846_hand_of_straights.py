from collections import Counter

def isNStraightHand(hand, groupSize):
    if len(hand) % groupSize != 0:
        return False
    count = Counter(hand)
    for start in sorted(count.keys()):
        cnt = count[start]
        if cnt > 0:
            for i in range(start, start + groupSize):
                if count[i] < cnt:
                    return False
                count[i] -= cnt
    return True
