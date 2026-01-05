def goodNodes(root):
    count = [0]

    def dfs(node, max_val):
        if not node:
            return
        if node.val >= max_val:
            count[0] += 1
        max_val = max(max_val, node.val)
        dfs(node.left, max_val)
        dfs(node.right, max_val)

    dfs(root, float('-inf'))
    return count[0]
