var characterReplacement = function(s, k) {
    const count = {};
    let left = 0, maxCount = 0, maxLen = 0;
    for (let right = 0; right < s.length; right++) {
        count[s[right]] = (count[s[right]] || 0) + 1;
        maxCount = Math.max(maxCount, count[s[right]]);
        while (right - left + 1 - maxCount > k) { count[s[left]]--; left++; }
        maxLen = Math.max(maxLen, right - left + 1);
    }
    return maxLen;
};
