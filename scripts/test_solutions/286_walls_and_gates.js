var wallsAndGates = function(rooms) {
    const m = rooms.length, n = rooms[0].length, queue = [];
    for (let i = 0; i < m; i++) for (let j = 0; j < n; j++) if (rooms[i][j] === 0) queue.push([i, j]);
    const dirs = [[1, 0], [-1, 0], [0, 1], [0, -1]];
    while (queue.length) {
        const [r, c] = queue.shift();
        for (const [dr, dc] of dirs) {
            const nr = r + dr, nc = c + dc;
            if (nr >= 0 && nr < m && nc >= 0 && nc < n && rooms[nr][nc] === 2147483647) {
                rooms[nr][nc] = rooms[r][c] + 1;
                queue.push([nr, nc]);
            }
        }
    }
};
