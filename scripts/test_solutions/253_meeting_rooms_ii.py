def minMeetingRooms(intervals):
    starts = sorted(i[0] for i in intervals)
    ends = sorted(i[1] for i in intervals)
    rooms, end_ptr = 0, 0
    for i in range(len(starts)):
        if starts[i] < ends[end_ptr]:
            rooms += 1
        else:
            end_ptr += 1
    return rooms
