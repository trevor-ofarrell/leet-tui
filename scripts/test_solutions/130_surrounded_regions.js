var solve = function(board) {
    const m = board.length, n = board[0].length;
    function dfs(i, j) {
        if (i < 0 || i >= m || j < 0 || j >= n || board[i][j] !== 'O') return;
        board[i][j] = 'T';
        dfs(i + 1, j); dfs(i - 1, j); dfs(i, j + 1); dfs(i, j - 1);
    }
    for (let i = 0; i < m; i++) { dfs(i, 0); dfs(i, n - 1); }
    for (let j = 0; j < n; j++) { dfs(0, j); dfs(m - 1, j); }
    for (let i = 0; i < m; i++) for (let j = 0; j < n; j++) {
        if (board[i][j] === 'O') board[i][j] = 'X';
        else if (board[i][j] === 'T') board[i][j] = 'O';
    }
};
