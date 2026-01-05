var isSubtree = function(root, subRoot) {
    function same(p, q) {
        if (!p && !q) return true;
        if (!p || !q || p.val !== q.val) return false;
        return same(p.left, q.left) && same(p.right, q.right);
    }
    if (!root) return false;
    if (same(root, subRoot)) return true;
    return isSubtree(root.left, subRoot) || isSubtree(root.right, subRoot);
};
