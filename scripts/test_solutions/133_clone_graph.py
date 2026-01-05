def cloneGraph(node):
    if node is None:
        return None
    visited = {}

    def dfs(n):
        if n.val in visited:
            return visited[n.val]
        clone = Node(n.val)
        visited[n.val] = clone
        for neighbor in sorted(n.neighbors, key=lambda x: x.val):
            clone.neighbors.append(dfs(neighbor))
        return clone

    return dfs(node)
