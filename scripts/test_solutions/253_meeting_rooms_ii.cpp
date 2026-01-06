class Solution {
public:
    int minMeetingRooms(vector<vector<int>>& intervals) {
        vector<int> starts, ends;
        for (const auto& i : intervals) {
            starts.push_back(i[0]);
            ends.push_back(i[1]);
        }

        sort(starts.begin(), starts.end());
        sort(ends.begin(), ends.end());

        int rooms = 0, end_ptr = 0;
        for (int i = 0; i < starts.size(); i++) {
            if (starts[i] < ends[end_ptr]) {
                rooms++;
            } else {
                end_ptr++;
            }
        }
        return rooms;
    }
};
