class Solution {
public:
    int minCostConnectPoints(vector<vector<int>>& points) {
        int n = points.size();
        vector<bool> visited(n, false);
        vector<int> min_dist(n, INT_MAX);
        min_dist[0] = 0;
        int cost = 0;

        for (int i = 0; i < n; i++) {
            int u = -1;
            int min_val = INT_MAX;
            for (int j = 0; j < n; j++) {
                if (!visited[j] && min_dist[j] < min_val) {
                    min_val = min_dist[j];
                    u = j;
                }
            }
            visited[u] = true;
            cost += min_val;
            for (int v = 0; v < n; v++) {
                if (!visited[v]) {
                    int dist = abs(points[u][0] - points[v][0]) + abs(points[u][1] - points[v][1]);
                    min_dist[v] = min(min_dist[v], dist);
                }
            }
        }

        return cost;
    }
};
