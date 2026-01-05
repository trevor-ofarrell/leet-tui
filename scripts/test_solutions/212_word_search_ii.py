def findWords(board, words):
    root = {}
    for w in words:
        node = root
        for c in w:
            if c not in node:
                node[c] = {}
            node = node[c]
        node['$'] = w

    result = []
    m, n = len(board), len(board[0])

    def dfs(i, j, node):
        if i < 0 or i >= m or j < 0 or j >= n:
            return
        c = board[i][j]
        if c == '#' or c not in node:
            return
        node = node[c]
        if '$' in node:
            result.append(node['$'])
            del node['$']
        board[i][j] = '#'
        dfs(i + 1, j, node)
        dfs(i - 1, j, node)
        dfs(i, j + 1, node)
        dfs(i, j - 1, node)
        board[i][j] = c

    for i in range(m):
        for j in range(n):
            dfs(i, j, root)
    return result
