var pacificAtlantic = function(heights) {
    const m = heights.length, n = heights[0].length;
    const pacific = Array(m).fill(null).map(() => Array(n).fill(false));
    const atlantic = Array(m).fill(null).map(() => Array(n).fill(false));
    function dfs(i, j, ocean) {
        ocean[i][j] = true;
        const dirs = [[1, 0], [-1, 0], [0, 1], [0, -1]];
        for (const [di, dj] of dirs) {
            const ni = i + di, nj = j + dj;
            if (ni >= 0 && ni < m && nj >= 0 && nj < n && !ocean[ni][nj] && heights[ni][nj] >= heights[i][j]) dfs(ni, nj, ocean);
        }
    }
    for (let i = 0; i < m; i++) { dfs(i, 0, pacific); dfs(i, n - 1, atlantic); }
    for (let j = 0; j < n; j++) { dfs(0, j, pacific); dfs(m - 1, j, atlantic); }
    const result = [];
    for (let i = 0; i < m; i++) for (let j = 0; j < n; j++) if (pacific[i][j] && atlantic[i][j]) result.push([i, j]);
    return result;
};
