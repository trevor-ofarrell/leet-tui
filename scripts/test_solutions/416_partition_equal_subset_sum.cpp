class Solution {
public:
    bool canPartition(vector<int>& nums) {
        int total = 0;
        for (int num : nums) total += num;
        if (total % 2 != 0) return false;

        int target = total / 2;
        unordered_set<int> dp;
        dp.insert(0);

        for (int num : nums) {
            unordered_set<int> newDp;
            for (int val : dp) {
                newDp.insert(val);
                newDp.insert(val + num);
            }
            if (newDp.count(target)) return true;
            dp = newDp;
        }
        return dp.count(target);
    }
};
