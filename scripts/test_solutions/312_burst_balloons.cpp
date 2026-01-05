class Solution {
public:
    int maxCoins(vector<int>& nums) {
        int n = nums.size();
        vector<int> balloons(n + 2, 1);
        for (int i = 0; i < n; i++) {
            balloons[i + 1] = nums[i];
        }
        n = balloons.size();

        vector<vector<int>> dp(n, vector<int>(n, 0));
        for (int length = 2; length < n; length++) {
            for (int left = 0; left < n - length; left++) {
                int right = left + length;
                for (int k = left + 1; k < right; k++) {
                    dp[left][right] = max(dp[left][right],
                        dp[left][k] + dp[k][right] + balloons[left] * balloons[k] * balloons[right]);
                }
            }
        }
        return dp[0][n - 1];
    }
};
