def kthSmallest(root, k):
    result = [0]
    count = [0]

    def inorder(node):
        if not node:
            return
        inorder(node.left)
        count[0] += 1
        if count[0] == k:
            result[0] = node.val
            return
        inorder(node.right)

    inorder(root)
    return result[0]
