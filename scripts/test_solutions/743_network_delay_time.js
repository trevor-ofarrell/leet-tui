var networkDelayTime = function(times, n, k) {
    const graph = new Map();
    for (const [u, v, w] of times) { if (!graph.has(u)) graph.set(u, []); graph.get(u).push([v, w]); }
    const dist = Array(n + 1).fill(Infinity);
    dist[k] = 0;
    const pq = [[0, k]];
    while (pq.length) {
        pq.sort((a, b) => a[0] - b[0]);
        const [d, u] = pq.shift();
        if (d > dist[u]) continue;
        for (const [v, w] of graph.get(u) || []) {
            if (dist[u] + w < dist[v]) { dist[v] = dist[u] + w; pq.push([dist[v], v]); }
        }
    }
    const maxDist = Math.max(...dist.slice(1));
    return maxDist === Infinity ? -1 : maxDist;
};
