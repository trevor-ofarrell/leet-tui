class TimeMap {
private:
    unordered_map<string, vector<pair<int, string>>> store;

public:
    TimeMap() {
    }

    void set(string key, string value, int timestamp) {
        store[key].push_back({timestamp, value});
    }

    string get(string key, int timestamp) {
        if (store.find(key) == store.end()) {
            return "";
        }
        auto& arr = store[key];
        int lo = 0;
        int hi = arr.size() - 1;
        string result = "";
        while (lo <= hi) {
            int mid = (lo + hi) / 2;
            if (arr[mid].first <= timestamp) {
                result = arr[mid].second;
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }
        return result;
    }
};
