class Solution {
public:
    int characterReplacement(string s, int k) {
        unordered_map<char, int> count;
        int left = 0, maxCount = 0, maxLen = 0;

        for (int right = 0; right < s.length(); right++) {
            count[s[right]]++;
            maxCount = max(maxCount, count[s[right]]);

            while (right - left + 1 - maxCount > k) {
                count[s[left]]--;
                left++;
            }
            maxLen = max(maxLen, right - left + 1);
        }
        return maxLen;
    }
};
