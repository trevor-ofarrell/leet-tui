var diameterOfBinaryTree = function(root) {
    let diameter = 0;
    function depth(node) {
        if (!node) return 0;
        const left = depth(node.left), right = depth(node.right);
        diameter = Math.max(diameter, left + right);
        return 1 + Math.max(left, right);
    }
    depth(root);
    return diameter;
};
