var longestIncreasingPath = function(matrix) {
    const m = matrix.length, n = matrix[0].length;
    const memo = Array(m).fill(null).map(() => Array(n).fill(0));
    function dfs(i, j) {
        if (memo[i][j]) return memo[i][j];
        let max = 1;
        const dirs = [[1, 0], [-1, 0], [0, 1], [0, -1]];
        for (const [di, dj] of dirs) {
            const ni = i + di, nj = j + dj;
            if (ni >= 0 && ni < m && nj >= 0 && nj < n && matrix[ni][nj] > matrix[i][j]) max = Math.max(max, 1 + dfs(ni, nj));
        }
        return memo[i][j] = max;
    }
    let result = 0;
    for (let i = 0; i < m; i++) for (let j = 0; j < n; j++) result = Math.max(result, dfs(i, j));
    return result;
};
