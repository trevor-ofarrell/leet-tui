var minMeetingRooms = function(intervals) {
    const starts = intervals.map(i => i[0]).sort((a, b) => a - b);
    const ends = intervals.map(i => i[1]).sort((a, b) => a - b);
    let rooms = 0, endPtr = 0;
    for (let i = 0; i < starts.length; i++) {
        if (starts[i] < ends[endPtr]) rooms++;
        else endPtr++;
    }
    return rooms;
};
