class Solution {
public:
    int longestIncreasingPath(vector<vector<int>>& matrix) {
        int m = matrix.size(), n = matrix[0].size();
        memo.assign(m, vector<int>(n, 0));

        int result = 0;
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                result = max(result, dfs(matrix, i, j));
            }
        }
        return result;
    }

private:
    vector<vector<int>> memo;
    int dirs[4][2] = {{1, 0}, {-1, 0}, {0, 1}, {0, -1}};

    int dfs(vector<vector<int>>& matrix, int i, int j) {
        if (memo[i][j]) return memo[i][j];
        int m = matrix.size(), n = matrix[0].size();
        int maxLen = 1;
        for (auto& d : dirs) {
            int ni = i + d[0], nj = j + d[1];
            if (ni >= 0 && ni < m && nj >= 0 && nj < n && matrix[ni][nj] > matrix[i][j]) {
                maxLen = max(maxLen, 1 + dfs(matrix, ni, nj));
            }
        }
        memo[i][j] = maxLen;
        return maxLen;
    }
};
