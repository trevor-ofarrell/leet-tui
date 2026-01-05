var canPartition = function(nums) {
    const sum = nums.reduce((a, b) => a + b, 0);
    if (sum % 2 !== 0) return false;
    const target = sum / 2;
    const dp = new Set([0]);
    for (const num of nums) {
        const newDp = new Set(dp);
        for (const val of dp) newDp.add(val + num);
        if (newDp.has(target)) return true;
        dp.clear(); for (const v of newDp) dp.add(v);
    }
    return dp.has(target);
};
