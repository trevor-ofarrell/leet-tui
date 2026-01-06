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

        while (!count.empty()) {
            int start = count.begin()->first;
            for (int i = start; i < start + groupSize; i++) {
                if (count.find(i) == count.end() || count[i] == 0) {
                    return false;
                }
                count[i]--;
                if (count[i] == 0) {
                    count.erase(i);
                }
            }
        }
        return true;
    }
};
