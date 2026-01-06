class Solution {
public:
    int lengthOfLongestSubstring(string s) {
        unordered_map<char, int> seen;
        int maxLen = 0;
        int left = 0;

        for (int right = 0; right < s.size(); right++) {
            char c = s[right];
            if (seen.count(c) && seen[c] >= left) {
                left = seen[c] + 1;
            }
            seen[c] = right;
            maxLen = max(maxLen, right - left + 1);
        }

        return maxLen;
    }
};
