class Solution {
public:
    int countSubstrings(string s) {
        int count = 0;
        int n = s.size();

        auto expand = [&](int l, int r) {
            while (l >= 0 && r < n && s[l] == s[r]) {
                count++;
                l--;
                r++;
            }
        };

        for (int i = 0; i < n; i++) {
            expand(i, i);
            expand(i, i + 1);
        }
        return count;
    }
};
