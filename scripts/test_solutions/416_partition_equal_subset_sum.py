def canPartition(nums):
    total = sum(nums)
    if total % 2 != 0:
        return False
    target = total // 2
    dp = {0}
    for num in nums:
        new_dp = set()
        for val in dp:
            new_dp.add(val)
            new_dp.add(val + num)
        if target in new_dp:
            return True
        dp = new_dp
    return target in dp
