var missingNumber = function(nums) {
    const n = nums.length;
    let xor = n;
    for (let i = 0; i < n; i++) {
        xor ^= i ^ nums[i];
    }
    return xor;
};
