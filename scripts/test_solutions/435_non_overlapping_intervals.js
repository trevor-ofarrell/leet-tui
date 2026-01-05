var eraseOverlapIntervals = function(intervals) {
    intervals.sort((a, b) => a[1] - b[1]);
    let count = 0, prevEnd = -Infinity;
    for (const [start, end] of intervals) {
        if (start >= prevEnd) prevEnd = end;
        else count++;
    }
    return count;
};
