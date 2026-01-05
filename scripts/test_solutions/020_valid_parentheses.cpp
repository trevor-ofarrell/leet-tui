class Solution {
public:
    bool isValid(string s) {
        stack<char> stk;
        unordered_map<char, char> mapping = {
            {')', '('}, {'}', '{'}, {']', '['}
        };
        unordered_set<char> opens = {'(', '{', '['};

        for (char c : s) {
            if (mapping.count(c)) {
                if (stk.empty() || stk.top() != mapping[c]) {
                    return false;
                }
                stk.pop();
            } else if (opens.count(c)) {
                stk.push(c);
            }
        }

        return stk.empty();
    }
};
