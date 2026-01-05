class Solution {
public:
    int countComponents(int n, vector<vector<int>>& edges) {
        parent.resize(n);
        for (int i = 0; i < n; i++) {
            parent[i] = i;
        }

        int count = n;
        for (auto& edge : edges) {
            int pa = find(edge[0]);
            int pb = find(edge[1]);
            if (pa != pb) {
                parent[pa] = pb;
                count--;
            }
        }
        return count;
    }

private:
    vector<int> parent;

    int find(int x) {
        if (parent[x] != x) {
            parent[x] = find(parent[x]);
        }
        return parent[x];
    }
};
