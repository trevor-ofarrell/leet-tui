var alienOrder = function(words) {
    const graph = new Map(), indegree = new Map();
    for (const w of words) for (const c of w) { graph.set(c, new Set()); indegree.set(c, 0); }
    for (let i = 0; i < words.length - 1; i++) {
        const w1 = words[i], w2 = words[i + 1];
        if (w1.length > w2.length && w1.startsWith(w2)) return '';
        for (let j = 0; j < Math.min(w1.length, w2.length); j++) {
            if (w1[j] !== w2[j]) {
                if (!graph.get(w1[j]).has(w2[j])) { graph.get(w1[j]).add(w2[j]); indegree.set(w2[j], indegree.get(w2[j]) + 1); }
                break;
            }
        }
    }
    const queue = [...indegree.keys()].filter(c => indegree.get(c) === 0), result = [];
    while (queue.length) {
        const c = queue.shift(); result.push(c);
        for (const next of graph.get(c)) { indegree.set(next, indegree.get(next) - 1); if (indegree.get(next) === 0) queue.push(next); }
    }
    return result.length === graph.size ? result.join('') : '';
};
