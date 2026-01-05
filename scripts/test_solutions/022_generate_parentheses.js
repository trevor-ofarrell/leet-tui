var generateParenthesis = function(n) {
    const result = [];
    function backtrack(open, close, path) {
        if (path.length === 2 * n) {
            result.push(path);
            return;
        }
        if (open < n) backtrack(open + 1, close, path + '(');
        if (close < open) backtrack(open, close + 1, path + ')');
    }
    backtrack(0, 0, '');
    return result;
};
