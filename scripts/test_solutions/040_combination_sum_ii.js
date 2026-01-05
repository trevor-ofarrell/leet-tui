var combinationSum2 = function(candidates, target) {
    candidates.sort((a, b) => a - b);
    const result = [];
    function backtrack(start, remaining, path) {
        if (remaining === 0) { result.push([...path]); return; }
        if (remaining < 0) return;
        for (let i = start; i < candidates.length; i++) {
            if (i > start && candidates[i] === candidates[i - 1]) continue;
            path.push(candidates[i]);
            backtrack(i + 1, remaining - candidates[i], path);
            path.pop();
        }
    }
    backtrack(0, target, []);
    return result;
};
