def findTargetSumWays(nums, target):
    total = sum(nums)
    if (total + target) % 2 != 0 or total + target < 0:
        return 0
    subset_sum = (total + target) // 2
    dp = [0] * (subset_sum + 1)
    dp[0] = 1
    for num in nums:
        for j in range(subset_sum, num - 1, -1):
            dp[j] += dp[j - num]
    return dp[subset_sum]
