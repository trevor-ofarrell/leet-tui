def longestIncreasingPath(matrix):
    m, n = len(matrix), len(matrix[0])
    memo = [[0] * n for _ in range(m)]

    def dfs(i, j):
        if memo[i][j]:
            return memo[i][j]
        max_len = 1
        for di, dj in [(1, 0), (-1, 0), (0, 1), (0, -1)]:
            ni, nj = i + di, j + dj
            if 0 <= ni < m and 0 <= nj < n and matrix[ni][nj] > matrix[i][j]:
                max_len = max(max_len, 1 + dfs(ni, nj))
        memo[i][j] = max_len
        return max_len

    result = 0
    for i in range(m):
        for j in range(n):
            result = max(result, dfs(i, j))
    return result
