class Solution {
public:
    vector<vector<string>> partition(string s) {
        vector<vector<string>> result;
        vector<string> path;

        function<bool(int, int)> isPalin = [&](int l, int r) {
            while (l < r) {
                if (s[l] != s[r]) return false;
                l++;
                r--;
            }
            return true;
        };

        function<void(int)> backtrack = [&](int start) {
            if (start == s.size()) {
                result.push_back(path);
                return;
            }
            for (int end = start; end < s.size(); end++) {
                if (isPalin(start, end)) {
                    path.push_back(s.substr(start, end - start + 1));
                    backtrack(end + 1);
                    path.pop_back();
                }
            }
        };

        backtrack(0);
        return result;
    }
};
