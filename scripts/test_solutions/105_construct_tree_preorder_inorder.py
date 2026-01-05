def buildTree(preorder, inorder):
    idx_map = {val: idx for idx, val in enumerate(inorder)}
    pre_idx = [0]

    def build(left, right):
        if left > right:
            return None
        val = preorder[pre_idx[0]]
        pre_idx[0] += 1
        node = TreeNode(val)
        idx = idx_map[val]
        node.left = build(left, idx - 1)
        node.right = build(idx + 1, right)
        return node

    return build(0, len(inorder) - 1)
