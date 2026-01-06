class Solution {
public:
    vector<int> topKFrequent(vector<int>& nums, int k) {
        unordered_map<int, int> freq;
        vector<int> order; // track insertion order
        for (int n : nums) {
            if (freq.find(n) == freq.end()) {
                order.push_back(n);
            }
            freq[n]++;
        }
        vector<vector<int>> bucket(nums.size() + 1);
        // Use insertion order
        for (int num : order) {
            bucket[freq[num]].push_back(num);
        }
        vector<int> result;
        for (int i = bucket.size() - 1; i >= 0 && result.size() < k; i--) {
            for (int num : bucket[i]) {
                result.push_back(num);
                if (result.size() == k) break;
            }
        }
        return result;
    }
};
