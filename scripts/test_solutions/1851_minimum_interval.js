var minInterval = function(intervals, queries) {
    intervals.sort((a, b) => a[0] - b[0]);
    const sortedQ = queries.map((q, i) => [q, i]).sort((a, b) => a[0] - b[0]);
    const result = Array(queries.length).fill(-1);
    const heap = [];
    let i = 0;
    for (const [q, idx] of sortedQ) {
        while (i < intervals.length && intervals[i][0] <= q) {
            heap.push([intervals[i][1] - intervals[i][0] + 1, intervals[i][1]]);
            heap.sort((a, b) => a[0] - b[0]);
            i++;
        }
        while (heap.length && heap[0][1] < q) heap.shift();
        if (heap.length) result[idx] = heap[0][0];
    }
    return result;
};
