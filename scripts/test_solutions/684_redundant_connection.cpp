class Solution {
public:
    vector<int> parent;

    int find(int x) {
        if (parent[x] != x) {
            parent[x] = find(parent[x]);
        }
        return parent[x];
    }

    vector<int> findRedundantConnection(vector<vector<int>>& edges) {
        int maxNode = 0;
        for (auto& edge : edges) {
            maxNode = max(maxNode, max(edge[0], edge[1]));
        }
        parent.resize(maxNode + 1);
        for (int i = 0; i <= maxNode; i++) {
            parent[i] = i;
        }

        for (auto& edge : edges) {
            int pa = find(edge[0]);
            int pb = find(edge[1]);
            if (pa == pb) {
                return edge;
            }
            parent[pa] = pb;
        }
        return {};
    }
};
