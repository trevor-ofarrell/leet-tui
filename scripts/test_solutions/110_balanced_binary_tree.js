var isBalanced = function(root) {
    function height(node) {
        if (!node) return 0;
        const left = height(node.left);
        const right = height(node.right);
        if (left === -1 || right === -1 || Math.abs(left - right) > 1) return -1;
        return 1 + Math.max(left, right);
    }
    return height(root) !== -1;
};
