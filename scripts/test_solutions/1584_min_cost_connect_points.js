var minCostConnectPoints = function(points) {
    const n = points.length, visited = new Set(), minDist = Array(n).fill(Infinity);
    minDist[0] = 0;
    let cost = 0;
    for (let i = 0; i < n; i++) {
        let u = -1, minVal = Infinity;
        for (let j = 0; j < n; j++) if (!visited.has(j) && minDist[j] < minVal) { minVal = minDist[j]; u = j; }
        visited.add(u);
        cost += minVal;
        for (let v = 0; v < n; v++) {
            if (!visited.has(v)) {
                const dist = Math.abs(points[u][0] - points[v][0]) + Math.abs(points[u][1] - points[v][1]);
                minDist[v] = Math.min(minDist[v], dist);
            }
        }
    }
    return cost;
};
