def countComponents(n, edges):
    parent = list(range(n))

    def find(x):
        if parent[x] != x:
            parent[x] = find(parent[x])
        return parent[x]

    count = n
    for a, b in edges:
        pa, pb = find(a), find(b)
        if pa != pb:
            parent[pa] = pb
            count -= 1

    return count
