class Solution {
public:
    string minWindow(string s, string t) {
        if (t.empty()) {
            return "";
        }

        unordered_map<char, int> need, window;
        for (char c : t) {
            need[c]++;
        }

        int have = 0, required = need.size();
        int left = 0, min_len = INT_MAX, min_start = 0;

        for (int right = 0; right < s.size(); right++) {
            char c = s[right];
            window[c]++;
            if (need.count(c) && window[c] == need[c]) {
                have++;
            }

            while (have == required) {
                if (right - left + 1 < min_len) {
                    min_len = right - left + 1;
                    min_start = left;
                }
                char lc = s[left];
                window[lc]--;
                if (need.count(lc) && window[lc] < need[lc]) {
                    have--;
                }
                left++;
            }
        }

        return min_len == INT_MAX ? "" : s.substr(min_start, min_len);
    }
};
