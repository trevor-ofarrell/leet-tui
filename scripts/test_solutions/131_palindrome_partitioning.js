var partition = function(s) {
    const result = [];
    function isPalin(l, r) { while (l < r) if (s[l++] !== s[r--]) return false; return true; }
    function backtrack(start, path) {
        if (start === s.length) { result.push([...path]); return; }
        for (let end = start; end < s.length; end++) {
            if (isPalin(start, end)) {
                path.push(s.substring(start, end + 1));
                backtrack(end + 1, path);
                path.pop();
            }
        }
    }
    backtrack(0, []);
    return result;
};
