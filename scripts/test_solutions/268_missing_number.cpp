class Solution {
public:
    int missingNumber(vector<int>& nums) {
        int xorVal = nums.size();
        for (int i = 0; i < nums.size(); i++) {
            xorVal ^= i ^ nums[i];
        }
        return xorVal;
    }
};
