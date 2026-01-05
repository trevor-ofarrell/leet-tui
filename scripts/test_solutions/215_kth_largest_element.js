var findKthLargest = function(nums, k) {
    function quickSelect(left, right) {
        const pivot = nums[right];
        let p = left;
        for (let i = left; i < right; i++) if (nums[i] >= pivot) [nums[i], nums[p]] = [nums[p], nums[i]], p++;
        [nums[p], nums[right]] = [nums[right], nums[p]];
        if (p === k - 1) return nums[p];
        return p < k - 1 ? quickSelect(p + 1, right) : quickSelect(left, p - 1);
    }
    return quickSelect(0, nums.length - 1);
};
