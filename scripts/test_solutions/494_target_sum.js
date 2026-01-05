var findTargetSumWays = function(nums, target) {
    const sum = nums.reduce((a, b) => a + b, 0);
    if ((sum + target) % 2 !== 0 || sum + target < 0) return 0;
    const subsetSum = (sum + target) / 2;
    const dp = Array(subsetSum + 1).fill(0);
    dp[0] = 1;
    for (const num of nums) {
        for (let j = subsetSum; j >= num; j--) dp[j] += dp[j - num];
    }
    return dp[subsetSum];
};
