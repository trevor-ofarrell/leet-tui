import heapq

def swimInWater(grid):
    n = len(grid)
    pq = [(grid[0][0], 0, 0)]
    visited = {(0, 0)}
    dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)]

    while pq:
        t, r, c = heapq.heappop(pq)
        if r == n - 1 and c == n - 1:
            return t
        for dr, dc in dirs:
            nr, nc = r + dr, c + dc
            if 0 <= nr < n and 0 <= nc < n and (nr, nc) not in visited:
                visited.add((nr, nc))
                heapq.heappush(pq, (max(t, grid[nr][nc]), nr, nc))
