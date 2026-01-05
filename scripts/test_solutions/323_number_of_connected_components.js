var countComponents = function(n, edges) {
    const parent = Array(n).fill(null).map((_, i) => i);
    function find(x) { if (parent[x] !== x) parent[x] = find(parent[x]); return parent[x]; }
    let count = n;
    for (const [a, b] of edges) {
        const pa = find(a), pb = find(b);
        if (pa !== pb) { parent[pa] = pb; count--; }
    }
    return count;
};
