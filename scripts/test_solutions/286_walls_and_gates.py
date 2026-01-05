from collections import deque

def wallsAndGates(rooms):
    if not rooms:
        return
    m, n = len(rooms), len(rooms[0])
    queue = deque()
    for i in range(m):
        for j in range(n):
            if rooms[i][j] == 0:
                queue.append((i, j))

    dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)]
    while queue:
        r, c = queue.popleft()
        for dr, dc in dirs:
            nr, nc = r + dr, c + dc
            if 0 <= nr < m and 0 <= nc < n and rooms[nr][nc] == 2147483647:
                rooms[nr][nc] = rooms[r][c] + 1
                queue.append((nr, nc))
