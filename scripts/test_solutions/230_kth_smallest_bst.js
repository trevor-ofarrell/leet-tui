var kthSmallest = function(root, k) {
    let count = 0, result = 0;
    function inorder(node) {
        if (!node) return;
        inorder(node.left);
        if (++count === k) { result = node.val; return; }
        inorder(node.right);
    }
    inorder(root);
    return result;
};
