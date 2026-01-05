var cloneGraph = function(node) {
    if (!node) return null;
    const map = new Map();
    function dfs(n) {
        if (map.has(n.val)) return map.get(n.val);
        const clone = { val: n.val, neighbors: [] };
        map.set(n.val, clone);
        for (const neighbor of n.neighbors) clone.neighbors.push(dfs(neighbor));
        return clone;
    }
    return dfs(node);
};
