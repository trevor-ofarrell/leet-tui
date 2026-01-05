def partitionLabels(s):
    last = {}
    for i, c in enumerate(s):
        last[c] = i
    result = []
    start = 0
    end = 0
    for i, c in enumerate(s):
        end = max(end, last[c])
        if i == end:
            result.append(end - start + 1)
            start = i + 1
    return result
