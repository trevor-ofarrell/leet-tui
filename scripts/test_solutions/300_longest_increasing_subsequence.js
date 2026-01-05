var lengthOfLIS = function(nums) {
    const tails = [];
    for (const num of nums) {
        let lo = 0, hi = tails.length;
        while (lo < hi) { const mid = Math.floor((lo + hi) / 2); if (tails[mid] < num) lo = mid + 1; else hi = mid; }
        tails[lo] = num;
    }
    return tails.length;
};
