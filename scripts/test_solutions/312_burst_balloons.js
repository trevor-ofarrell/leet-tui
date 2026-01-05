var maxCoins = function(nums) {
    nums = [1, ...nums, 1];
    const n = nums.length;
    const dp = Array(n).fill(null).map(() => Array(n).fill(0));
    for (let len = 2; len < n; len++) {
        for (let left = 0; left < n - len; left++) {
            const right = left + len;
            for (let k = left + 1; k < right; k++) {
                dp[left][right] = Math.max(dp[left][right], dp[left][k] + dp[k][right] + nums[left] * nums[k] * nums[right]);
            }
        }
    }
    return dp[0][n - 1];
};
