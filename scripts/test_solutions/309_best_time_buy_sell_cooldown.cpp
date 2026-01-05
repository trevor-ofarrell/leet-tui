class Solution {
public:
    int maxProfit(vector<int>& prices) {
        int sold = 0, held = INT_MIN, rest = 0;
        for (int p : prices) {
            int prev_sold = sold;
            sold = held + p;
            held = max(held, rest - p);
            rest = max(rest, prev_sold);
        }
        return max(sold, rest);
    }
};
