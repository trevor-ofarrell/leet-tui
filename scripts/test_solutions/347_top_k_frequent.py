def topKFrequent(nums, k):
    freq = {}
    for n in nums:
        freq[n] = freq.get(n, 0) + 1
    bucket = [[] for _ in range(len(nums) + 1)]
    for num, count in freq.items():
        bucket[count].append(num)
    result = []
    for i in range(len(bucket) - 1, -1, -1):
        result.extend(bucket[i])
        if len(result) >= k:
            break
    return result[:k]
