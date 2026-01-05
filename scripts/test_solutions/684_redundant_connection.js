var findRedundantConnection = function(edges) {
    const parent = Array(edges.length + 1).fill(null).map((_, i) => i);
    function find(x) { if (parent[x] !== x) parent[x] = find(parent[x]); return parent[x]; }
    for (const [a, b] of edges) {
        const pa = find(a), pb = find(b);
        if (pa === pb) return [a, b];
        parent[pa] = pb;
    }
};
