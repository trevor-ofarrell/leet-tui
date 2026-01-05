class Solution {
public:
    string alienOrder(vector<string>& words) {
        unordered_map<char, unordered_set<char>> graph;
        unordered_map<char, int> indegree;

        // Initialize all characters with indegree 0
        for (const string& w : words) {
            for (char c : w) {
                indegree[c] = 0;
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

        // Topological sort using BFS
        queue<char> q;
        for (auto& p : indegree) {
            if (p.second == 0) {
                q.push(p.first);
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

        return result.size() == indegree.size() ? result : "";
    }
};
