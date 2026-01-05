def diameterOfBinaryTree(root):
    diameter = [0]

    def depth(node):
        if not node:
            return 0
        left = depth(node.left)
        right = depth(node.right)
        diameter[0] = max(diameter[0], left + right)
        return 1 + max(left, right)

    depth(root)
    return diameter[0]
