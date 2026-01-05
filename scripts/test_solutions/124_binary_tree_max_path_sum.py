def maxPathSum(root):
    max_sum = [float('-inf')]

    def dfs(node):
        if not node:
            return 0
        left = max(0, dfs(node.left))
        right = max(0, dfs(node.right))
        max_sum[0] = max(max_sum[0], node.val + left + right)
        return node.val + max(left, right)

    dfs(root)
    return max_sum[0]
