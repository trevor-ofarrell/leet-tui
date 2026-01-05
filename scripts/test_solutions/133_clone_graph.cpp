struct Node {
    int val;
    vector<Node*> neighbors;
    Node() : val(0), neighbors(vector<Node*>()) {}
    Node(int _val) : val(_val), neighbors(vector<Node*>()) {}
    Node(int _val, vector<Node*> _neighbors) : val(_val), neighbors(_neighbors) {}
};

class Solution {
public:
    vector<vector<int>> cloneGraph(vector<vector<int>>& adjList) {
        if (adjList.empty()) {
            return {};
        }

        int n = adjList.size();
        vector<Node*> nodes(n + 1, nullptr);
        vector<Node*> clones(n + 1, nullptr);

        // Build original graph from adjacency list
        for (int i = 1; i <= n; i++) {
            nodes[i] = new Node(i);
        }
        for (int i = 1; i <= n; i++) {
            for (int neighbor : adjList[i - 1]) {
                nodes[i]->neighbors.push_back(nodes[neighbor]);
            }
        }

        // Clone: First create all clone nodes
        for (int i = 1; i <= n; i++) {
            clones[i] = new Node(i);
        }

        // Clone: Then link neighbors in the correct order
        for (int i = 1; i <= n; i++) {
            for (Node* neighbor : nodes[i]->neighbors) {
                clones[i]->neighbors.push_back(clones[neighbor->val]);
            }
        }

        // Convert cloned graph back to adjacency list (sorted neighbors)
        vector<vector<int>> result(n);
        for (int i = 1; i <= n; i++) {
            for (Node* neighbor : clones[i]->neighbors) {
                result[i - 1].push_back(neighbor->val);
            }
            sort(result[i - 1].begin(), result[i - 1].end());
        }

        return result;
    }
};
