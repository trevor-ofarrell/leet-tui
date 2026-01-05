var findItinerary = function(tickets) {
    const graph = new Map();
    for (const [from, to] of tickets) {
        if (!graph.has(from)) graph.set(from, []);
        graph.get(from).push(to);
    }
    for (const [_, dests] of graph) dests.sort().reverse();
    const result = [];
    function dfs(airport) {
        const dests = graph.get(airport);
        while (dests && dests.length) dfs(dests.pop());
        result.push(airport);
    }
    dfs('JFK');
    return result.reverse();
};
