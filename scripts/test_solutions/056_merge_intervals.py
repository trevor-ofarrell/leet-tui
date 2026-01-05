def merge(intervals):
    if not intervals:
        return []
    # Handle nested list input: [[[1,3],[2,6]]] -> [[1,3],[2,6]]
    if intervals and isinstance(intervals[0], list) and intervals[0] and isinstance(intervals[0][0], list):
        intervals = intervals[0]
    intervals.sort(key=lambda x: x[0])
    result = [intervals[0][:]]
    for i in range(1, len(intervals)):
        last = result[-1]
        if intervals[i][0] <= last[1]:
            last[1] = max(last[1], intervals[i][1])
        else:
            result.append(intervals[i][:])
    return result
