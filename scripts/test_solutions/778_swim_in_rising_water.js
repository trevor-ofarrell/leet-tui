var swimInWater = function(grid) {
    const n = grid.length;
    const pq = [[grid[0][0], 0, 0]];
    const visited = new Set(['0,0']);
    const dirs = [[1, 0], [-1, 0], [0, 1], [0, -1]];
    while (pq.length) {
        pq.sort((a, b) => a[0] - b[0]);
        const [t, r, c] = pq.shift();
        if (r === n - 1 && c === n - 1) return t;
        for (const [dr, dc] of dirs) {
            const nr = r + dr, nc = c + dc, key = `${nr},${nc}`;
            if (nr >= 0 && nr < n && nc >= 0 && nc < n && !visited.has(key)) {
                visited.add(key);
                pq.push([Math.max(t, grid[nr][nc]), nr, nc]);
            }
        }
    }
};
