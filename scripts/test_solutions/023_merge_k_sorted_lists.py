import heapq

def mergeKLists(lists, *args):
    # Handle input as list of arrays (from test harness)
    if not lists:
        return []

    # Filter out empty lists
    lists = [l for l in lists if l]
    if not lists:
        return []

    # Use a min-heap to merge k sorted lists
    heap = []
    for i, lst in enumerate(lists):
        if lst:
            heapq.heappush(heap, (lst[0], i, 0))

    result = []
    while heap:
        val, list_idx, elem_idx = heapq.heappop(heap)
        result.append(val)
        if elem_idx + 1 < len(lists[list_idx]):
            next_val = lists[list_idx][elem_idx + 1]
            heapq.heappush(heap, (next_val, list_idx, elem_idx + 1))

    return result
