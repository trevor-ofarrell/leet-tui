class Solution {
public:
    int leastInterval(vector<char>& tasks, int n) {
        vector<int> freq(26, 0);
        for (char t : tasks) {
            freq[t - 'A']++;
        }
        int maxFreq = *max_element(freq.begin(), freq.end());
        int maxCount = count(freq.begin(), freq.end(), maxFreq);
        return max((int)tasks.size(), (maxFreq - 1) * (n + 1) + maxCount);
    }
};
