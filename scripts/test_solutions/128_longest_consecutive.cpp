class Solution {
public:
    int longestConsecutive(vector<int>& nums) {
        unordered_set<int> numSet(nums.begin(), nums.end());
        int maxLen = 0;
        for (int num : numSet) {
            if (numSet.find(num - 1) == numSet.end()) {
                int curr = num;
                int length = 1;
                while (numSet.find(curr + 1) != numSet.end()) {
                    curr++;
                    length++;
                }
                maxLen = max(maxLen, length);
            }
        }
        return maxLen;
    }
};
