var findOrder = function(numCourses, prerequisites) {
    const graph = Array(numCourses).fill(null).map(() => []);
    const indegree = Array(numCourses).fill(0);
    for (const [a, b] of prerequisites) { graph[b].push(a); indegree[a]++; }
    const queue = [], result = [];
    for (let i = 0; i < numCourses; i++) if (indegree[i] === 0) queue.push(i);
    while (queue.length) {
        const course = queue.shift();
        result.push(course);
        for (const next of graph[course]) { if (--indegree[next] === 0) queue.push(next); }
    }
    return result.length === numCourses ? result : [];
};
