def findRedundantConnection(edges):
    # Use max node value to size parent array (1-indexed nodes)
    max_node = max(max(a, b) for a, b in edges)
    parent = list(range(max_node + 1))

    def find(x):
        if parent[x] != x:
            parent[x] = find(parent[x])
        return parent[x]

    for a, b in edges:
        pa, pb = find(a), find(b)
        if pa == pb:
            return [a, b]
        parent[pa] = pb
    return []
