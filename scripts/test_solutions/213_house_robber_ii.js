var rob = function(nums) {
    if (nums.length === 1) return nums[0];
    function robRange(start, end) {
        let prev2 = 0, prev1 = 0;
        for (let i = start; i <= end; i++) {
            const curr = Math.max(prev1, prev2 + nums[i]);
            prev2 = prev1; prev1 = curr;
        }
        return prev1;
    }
    return Math.max(robRange(0, nums.length - 2), robRange(1, nums.length - 1));
};
