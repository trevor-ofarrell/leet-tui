def pacificAtlantic(heights):
    if not heights:
        return []
    # Handle 1D array as 1xN matrix
    if not isinstance(heights[0], list):
        heights = [heights]
    m, n = len(heights), len(heights[0])
    pacific = [[False] * n for _ in range(m)]
    atlantic = [[False] * n for _ in range(m)]

    def dfs(i, j, ocean):
        ocean[i][j] = True
        for di, dj in [(1, 0), (-1, 0), (0, 1), (0, -1)]:
            ni, nj = i + di, j + dj
            if 0 <= ni < m and 0 <= nj < n and not ocean[ni][nj] and heights[ni][nj] >= heights[i][j]:
                dfs(ni, nj, ocean)

    for i in range(m):
        dfs(i, 0, pacific)
        dfs(i, n - 1, atlantic)
    for j in range(n):
        dfs(0, j, pacific)
        dfs(m - 1, j, atlantic)

    return [[i, j] for i in range(m) for j in range(n) if pacific[i][j] and atlantic[i][j]]
