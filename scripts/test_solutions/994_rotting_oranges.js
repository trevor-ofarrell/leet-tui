var orangesRotting = function(grid) {
    const m = grid.length, n = grid[0].length, queue = [];
    let fresh = 0;
    for (let i = 0; i < m; i++) for (let j = 0; j < n; j++) {
        if (grid[i][j] === 2) queue.push([i, j]);
        else if (grid[i][j] === 1) fresh++;
    }
    if (fresh === 0) return 0;
    const dirs = [[1, 0], [-1, 0], [0, 1], [0, -1]];
    let minutes = 0;
    while (queue.length) {
        const size = queue.length;
        for (let i = 0; i < size; i++) {
            const [r, c] = queue.shift();
            for (const [dr, dc] of dirs) {
                const nr = r + dr, nc = c + dc;
                if (nr >= 0 && nr < m && nc >= 0 && nc < n && grid[nr][nc] === 1) {
                    grid[nr][nc] = 2; fresh--; queue.push([nr, nc]);
                }
            }
        }
        if (queue.length) minutes++;
    }
    return fresh === 0 ? minutes : -1;
};
