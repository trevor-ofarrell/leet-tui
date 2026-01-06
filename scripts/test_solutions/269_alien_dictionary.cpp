class Solution {
public:
    string alienOrder(vector<string>& words) {
        unordered_map<char, unordered_set<char>> graph;
        unordered_map<char, int> indegree;
        vector<char> charOrder;  // Track order of first appearance
        unordered_set<char> seen;

        // Initialize all characters and track order
        for (const string& w : words) {
            for (char c : w) {
                if (seen.find(c) == seen.end()) {
                    seen.insert(c);
                    charOrder.push_back(c);
                    indegree[c] = 0;
                }
            }
        }

        // Build graph
        for (int i = 0; i < words.size() - 1; i++) {
            const string& w1 = words[i];
            const string& w2 = words[i + 1];

            // Check for invalid case: w1 is prefix of w2 but longer
            if (w1.size() > w2.size() && w1.substr(0, w2.size()) == w2) {
                return "";
            }

            int minLen = min(w1.size(), w2.size());
            for (int j = 0; j < minLen; j++) {
                if (w1[j] != w2[j]) {
                    if (graph[w1[j]].find(w2[j]) == graph[w1[j]].end()) {
                        graph[w1[j]].insert(w2[j]);
                        indegree[w2[j]]++;
                    }
                    break;
                }
            }
        }

        // Topological sort using BFS, respecting first appearance order
        queue<char> q;
        for (char c : charOrder) {
            if (indegree[c] == 0) {
                q.push(c);
            }
        }

        string result;
        while (!q.empty()) {
            char c = q.front();
            q.pop();
            result += c;
            for (char next_c : graph[c]) {
                indegree[next_c]--;
                if (indegree[next_c] == 0) {
                    q.push(next_c);
                }
            }
        }

        return result.size() == seen.size() ? result : "";
    }
};
