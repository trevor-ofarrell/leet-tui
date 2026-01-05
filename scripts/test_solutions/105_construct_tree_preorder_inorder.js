var buildTree = function(preorder, inorder) {
    const map = new Map();
    inorder.forEach((val, idx) => map.set(val, idx));
    let preIdx = 0;
    function build(left, right) {
        if (left > right) return null;
        const val = preorder[preIdx++];
        const node = { val, left: null, right: null };
        const idx = map.get(val);
        node.left = build(left, idx - 1);
        node.right = build(idx + 1, right);
        return node;
    }
    return build(0, inorder.length - 1);
};
