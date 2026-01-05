var jump = function(nums) {
    let jumps = 0, currEnd = 0, farthest = 0;
    for (let i = 0; i < nums.length - 1; i++) {
        farthest = Math.max(farthest, i + nums[i]);
        if (i === currEnd) {
            jumps++;
            currEnd = farthest;
        }
    }
    return jumps;
};
