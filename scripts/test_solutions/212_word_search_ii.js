var findWords = function(board, words) {
    const root = {};
    for (const w of words) { let n = root; for (const c of w) { if (!n[c]) n[c] = {}; n = n[c]; } n.word = w; }
    const result = [], m = board.length, n = board[0].length;
    function dfs(i, j, node) {
        if (i < 0 || i >= m || j < 0 || j >= n) return;
        const c = board[i][j];
        if (c === '#' || !node[c]) return;
        node = node[c];
        if (node.word) { result.push(node.word); node.word = null; }
        board[i][j] = '#';
        dfs(i + 1, j, node); dfs(i - 1, j, node); dfs(i, j + 1, node); dfs(i, j - 1, node);
        board[i][j] = c;
    }
    for (let i = 0; i < m; i++) for (let j = 0; j < n; j++) dfs(i, j, root);
    return result;
};
