class Solution {
public:
    bool validTree(int n, vector<vector<int>>& edges) {
        if (n == 0) return false;
        if (edges.size() != n - 1) return false;
        if (edges.empty()) return n == 1;

        vector<int> parent(n);
        for (int i = 0; i < n; i++) {
            parent[i] = i;
        }

        for (const auto& e : edges) {
            int pa = find(parent, e[0]);
            int pb = find(parent, e[1]);
            if (pa == pb) return false;
            parent[pa] = pb;
        }

        return true;
    }

private:
    int find(vector<int>& parent, int x) {
        if (parent[x] != x) {
            parent[x] = find(parent, parent[x]);
        }
        return parent[x];
    }
};
