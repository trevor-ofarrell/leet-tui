var isValid = function(s) {
    const stack = [];
    const map = { ')': '(', '}': '{', ']': '[' };
    const opens = new Set(['(', '{', '[']);
    for (const c of s) {
        if (c in map) {
            if (stack.pop() !== map[c]) return false;
        } else if (opens.has(c)) {
            stack.push(c);
        }
        // Ignore non-bracket characters
    }
    return stack.length === 0;
};
