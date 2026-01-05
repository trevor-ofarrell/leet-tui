import heapq

def findKthLargest(nums, k):
    return heapq.nlargest(k, nums)[-1]
