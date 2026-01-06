class Solution {
public:
    int numDecodings(string s) {
        if (s.empty()) {
            return 1;
        }
        if (s[0] == '0') {
            return 0;
        }
        int n = s.size();
        int prev2 = 1, prev1 = 1;
        for (int i = 1; i < n; i++) {
            int curr = 0;
            if (s[i] != '0') {
                curr = prev1;
            }
            int twoDigit = (s[i - 1] - '0') * 10 + (s[i] - '0');
            if (twoDigit >= 10 && twoDigit <= 26) {
                curr += prev2;
            }
            prev2 = prev1;
            prev1 = curr;
        }
        return prev1;
    }
};
