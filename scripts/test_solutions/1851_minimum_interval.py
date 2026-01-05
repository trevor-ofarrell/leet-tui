import heapq

def minInterval(intervals, queries):
    intervals = sorted(intervals, key=lambda x: x[0])
    sorted_q = sorted(enumerate(queries), key=lambda x: x[1])
    result = [-1] * len(queries)
    heap = []
    i = 0

    for idx, q in sorted_q:
        while i < len(intervals) and intervals[i][0] <= q:
            heapq.heappush(heap, (intervals[i][1] - intervals[i][0] + 1, intervals[i][1]))
            i += 1
        while heap and heap[0][1] < q:
            heapq.heappop(heap)
        if heap:
            result[idx] = heap[0][0]

    return result
