class Solution {
public:
    bool isNStraightHand(vector<int>& hand, int groupSize) {
        if (hand.size() % groupSize != 0) {
            return false;
        }
        map<int, int> count;
        for (int card : hand) {
            count[card]++;
        }

        for (auto& [start, cnt] : count) {
            if (cnt > 0) {
                for (int i = start; i < start + groupSize; i++) {
                    if (count[i] < cnt) {
                        return false;
                    }
                    count[i] -= cnt;
                }
            }
        }
        return true;
    }
};
