var findMedianSortedArrays = function(nums1, nums2) {
    if (nums1.length > nums2.length) [nums1, nums2] = [nums2, nums1];
    let m = nums1.length, n = nums2.length;
    let lo = 0, hi = m;
    while (lo <= hi) {
        let i = Math.floor((lo + hi) / 2);
        let j = Math.floor((m + n + 1) / 2) - i;
        let maxLeft1 = i === 0 ? -Infinity : nums1[i - 1];
        let minRight1 = i === m ? Infinity : nums1[i];
        let maxLeft2 = j === 0 ? -Infinity : nums2[j - 1];
        let minRight2 = j === n ? Infinity : nums2[j];
        if (maxLeft1 <= minRight2 && maxLeft2 <= minRight1) {
            if ((m + n) % 2 === 0) {
                return (Math.max(maxLeft1, maxLeft2) + Math.min(minRight1, minRight2)) / 2;
            }
            return Math.max(maxLeft1, maxLeft2);
        } else if (maxLeft1 > minRight2) {
            hi = i - 1;
        } else {
            lo = i + 1;
        }
    }
};
