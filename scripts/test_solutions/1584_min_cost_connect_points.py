def minCostConnectPoints(points):
    n = len(points)
    visited = set()
    min_dist = [float('inf')] * n
    min_dist[0] = 0
    cost = 0

    for _ in range(n):
        u = -1
        min_val = float('inf')
        for j in range(n):
            if j not in visited and min_dist[j] < min_val:
                min_val = min_dist[j]
                u = j
        visited.add(u)
        cost += min_val
        for v in range(n):
            if v not in visited:
                dist = abs(points[u][0] - points[v][0]) + abs(points[u][1] - points[v][1])
                min_dist[v] = min(min_dist[v], dist)

    return cost
