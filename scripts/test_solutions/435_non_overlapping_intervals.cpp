class Solution {
public:
    int eraseOverlapIntervals(vector<vector<int>>& intervals) {
        sort(intervals.begin(), intervals.end(), [](auto& a, auto& b) {
            return a[1] < b[1];
        });

        int count = 0;
        int prevEnd = INT_MIN;

        for (auto& interval : intervals) {
            if (interval[0] >= prevEnd) {
                prevEnd = interval[1];
            } else {
                count++;
            }
        }
        return count;
    }
};
