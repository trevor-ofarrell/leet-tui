class Solution {
public:
    int maxProfit(vector<int>& prices) {
        if (prices.empty()) return 0;
        int sold = 0, held = INT_MIN / 2, rest = 0;  // Use INT_MIN/2 to avoid overflow
        for (int p : prices) {
            int prev_sold = sold;
            sold = held + p;
            held = max(held, rest - p);
            rest = max(rest, prev_sold);
        }
        return max(sold, rest);
    }
};
