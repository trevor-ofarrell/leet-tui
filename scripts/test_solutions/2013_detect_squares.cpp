class DetectSquares {
private:
    map<pair<int, int>, int> points;

public:
    DetectSquares() {
    }

    void add(vector<int> point) {
        points[{point[0], point[1]}]++;
    }

    int count(vector<int> point) {
        int x1 = point[0];
        int y1 = point[1];
        int result = 0;

        for (auto& [p, cnt] : points) {
            int x2 = p.first;
            int y2 = p.second;
            if (abs(x2 - x1) != abs(y2 - y1) || x1 == x2) {
                continue;
            }
            int cnt1 = 0;
            int cnt2 = 0;
            auto it1 = points.find({x1, y2});
            if (it1 != points.end()) {
                cnt1 = it1->second;
            }
            auto it2 = points.find({x2, y1});
            if (it2 != points.end()) {
                cnt2 = it2->second;
            }
            result += cnt * cnt1 * cnt2;
        }
        return result;
    }
};
