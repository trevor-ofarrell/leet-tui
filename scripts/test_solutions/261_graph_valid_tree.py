def validTree(n, edges):
    if n == 0:
        return False
    if len(edges) != n - 1:
        return False
    if not edges:
        return n == 1
    parent = list(range(n))

    def find(x):
        if parent[x] != x:
            parent[x] = find(parent[x])
        return parent[x]

    for a, b in edges:
        pa, pb = find(a), find(b)
        if pa == pb:
            return False
        parent[pa] = pb

    return True
