class Solution {
public:
    vector<string> letterCombinations(string digits) {
        if (digits.empty()) return {};

        unordered_map<char, string> mapping = {
            {'2', "abc"}, {'3', "def"}, {'4', "ghi"}, {'5', "jkl"},
            {'6', "mno"}, {'7', "pqrs"}, {'8', "tuv"}, {'9', "wxyz"}
        };

        vector<string> result;

        function<void(int, string)> backtrack = [&](int idx, string path) {
            if (idx == digits.size()) {
                result.push_back(path);
                return;
            }
            for (char c : mapping[digits[idx]]) {
                backtrack(idx + 1, path + c);
            }
        };

        backtrack(0, "");
        return result;
    }
};
