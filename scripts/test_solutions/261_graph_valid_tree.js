var validTree = function(n, edges) {
    if (edges.length !== n - 1) return false;
    const parent = Array(n).fill(null).map((_, i) => i);
    function find(x) { if (parent[x] !== x) parent[x] = find(parent[x]); return parent[x]; }
    for (const [a, b] of edges) {
        const pa = find(a), pb = find(b);
        if (pa === pb) return false;
        parent[pa] = pb;
    }
    return true;
};
