var goodNodes = function(root) {
    let count = 0;
    function dfs(node, maxVal) {
        if (!node) return;
        if (node.val >= maxVal) count++;
        maxVal = Math.max(maxVal, node.val);
        dfs(node.left, maxVal);
        dfs(node.right, maxVal);
    }
    dfs(root, -Infinity);
    return count;
};
