var longestConsecutive = function(nums) {
    const set = new Set(nums);
    let maxLen = 0;
    for (const num of set) {
        if (!set.has(num - 1)) {
            let curr = num, len = 1;
            while (set.has(curr + 1)) { curr++; len++; }
            maxLen = Math.max(maxLen, len);
        }
    }
    return maxLen;
};
