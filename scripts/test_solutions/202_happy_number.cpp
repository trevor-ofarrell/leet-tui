class Solution {
public:
    bool isHappy(int n) {
        unordered_set<int> seen;
        while (n != 1 && seen.find(n) == seen.end()) {
            seen.insert(n);
            int total = 0;
            while (n > 0) {
                int digit = n % 10;
                total += digit * digit;
                n /= 10;
            }
            n = total;
        }
        return n == 1;
    }
};
