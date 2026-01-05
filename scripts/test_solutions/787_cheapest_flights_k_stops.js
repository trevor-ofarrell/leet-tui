var findCheapestPrice = function(n, flights, src, dst, k) {
    let prices = Array(n).fill(Infinity);
    prices[src] = 0;
    for (let i = 0; i <= k; i++) {
        const temp = [...prices];
        for (const [u, v, p] of flights) {
            if (prices[u] !== Infinity) temp[v] = Math.min(temp[v], prices[u] + p);
        }
        prices = temp;
    }
    return prices[dst] === Infinity ? -1 : prices[dst];
};
