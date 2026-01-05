var serialize = function(root) {
    const result = [];
    function dfs(node) { if (!node) result.push('null'); else { result.push(node.val); dfs(node.left); dfs(node.right); } }
    dfs(root);
    return result.join(',');
};
var deserialize = function(data) {
    const vals = data.split(',');
    let i = 0;
    function dfs() {
        if (vals[i] === 'null') { i++; return null; }
        const node = { val: parseInt(vals[i++]), left: null, right: null };
        node.left = dfs(); node.right = dfs();
        return node;
    }
    return dfs();
};
