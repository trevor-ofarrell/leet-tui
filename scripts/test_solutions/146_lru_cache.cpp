class LRUCache {
public:
    LRUCache(int capacity) : capacity(capacity) {}

    int get(int key) {
        auto it = cache.find(key);
        if (it == cache.end()) {
            return -1;
        }
        // Move to front (most recently used)
        order.splice(order.begin(), order, it->second.second);
        return it->second.first;
    }

    void put(int key, int value) {
        auto it = cache.find(key);
        if (it != cache.end()) {
            // Update value and move to front
            it->second.first = value;
            order.splice(order.begin(), order, it->second.second);
        } else {
            // Add new entry
            if (cache.size() >= capacity) {
                // Remove least recently used (back of list)
                int lruKey = order.back();
                order.pop_back();
                cache.erase(lruKey);
            }
            order.push_front(key);
            cache[key] = {value, order.begin()};
        }
    }

private:
    int capacity;
    list<int> order;  // Front = most recently used, back = least recently used
    unordered_map<int, pair<int, list<int>::iterator>> cache;  // key -> {value, iterator}
};
