from functools import reduce

def singleNumber(nums):
    return reduce(lambda a, b: a ^ b, nums)
