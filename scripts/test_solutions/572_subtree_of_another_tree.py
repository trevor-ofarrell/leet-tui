def isSubtree(root, subRoot):
    def same(p, q):
        if not p and not q:
            return True
        if not p or not q or p.val != q.val:
            return False
        return same(p.left, q.left) and same(p.right, q.right)

    if not root:
        return False
    if same(root, subRoot):
        return True
    return isSubtree(root.left, subRoot) or isSubtree(root.right, subRoot)
