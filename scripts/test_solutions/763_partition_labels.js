var partitionLabels = function(s) {
    const last = {};
    for (let i = 0; i < s.length; i++) last[s[i]] = i;
    const result = [];
    let start = 0, end = 0;
    for (let i = 0; i < s.length; i++) {
        end = Math.max(end, last[s[i]]);
        if (i === end) { result.push(end - start + 1); start = i + 1; }
    }
    return result;
};
