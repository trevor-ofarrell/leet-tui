class Solution {
public:
    vector<string> generateParenthesis(int n) {
        vector<string> result;

        function<void(int, int, string)> backtrack = [&](int open_count, int close_count, string path) {
            if (path.size() == 2 * n) {
                result.push_back(path);
                return;
            }
            if (open_count < n) {
                backtrack(open_count + 1, close_count, path + '(');
            }
            if (close_count < open_count) {
                backtrack(open_count, close_count + 1, path + ')');
            }
        };

        backtrack(0, 0, "");
        return result;
    }
};
