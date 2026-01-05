var canFinish = function(numCourses, prerequisites) {
    const graph = Array(numCourses).fill(null).map(() => []);
    const indegree = Array(numCourses).fill(0);
    for (const [a, b] of prerequisites) { graph[b].push(a); indegree[a]++; }
    const queue = [];
    for (let i = 0; i < numCourses; i++) if (indegree[i] === 0) queue.push(i);
    let count = 0;
    while (queue.length) {
        const course = queue.shift();
        count++;
        for (const next of graph[course]) { if (--indegree[next] === 0) queue.push(next); }
    }
    return count === numCourses;
};
