def missingNumber(nums):
    n = len(nums)
    xor = n
    for i in range(n):
        xor ^= i ^ nums[i]
    return xor
