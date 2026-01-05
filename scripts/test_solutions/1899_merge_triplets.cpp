class Solution {
public:
    bool mergeTriplets(vector<vector<int>>& triplets, vector<int>& target) {
        set<int> good;
        for (auto& t : triplets) {
            if (t[0] <= target[0] && t[1] <= target[1] && t[2] <= target[2]) {
                if (t[0] == target[0]) good.insert(0);
                if (t[1] == target[1]) good.insert(1);
                if (t[2] == target[2]) good.insert(2);
            }
        }
        return good.size() == 3;
    }
};
