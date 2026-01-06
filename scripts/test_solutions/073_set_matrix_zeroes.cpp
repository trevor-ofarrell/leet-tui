class Solution {
public:
    vector<vector<int>> setZeroes(vector<vector<int>>& matrix) {
        int m = matrix.size(), n = matrix[0].size();
        bool first_row_zero = false, first_col_zero = false;

        for (int j = 0; j < n; j++) {
            if (matrix[0][j] == 0) {
                first_row_zero = true;
                break;
            }
        }
        for (int i = 0; i < m; i++) {
            if (matrix[i][0] == 0) {
                first_col_zero = true;
                break;
            }
        }

        for (int i = 1; i < m; i++) {
            for (int j = 1; j < n; j++) {
                if (matrix[i][j] == 0) {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }

        for (int i = 1; i < m; i++) {
            for (int j = 1; j < n; j++) {
                if (matrix[i][0] == 0 || matrix[0][j] == 0) {
                    matrix[i][j] = 0;
                }
            }
        }

        if (first_row_zero) {
            for (int j = 0; j < n; j++) {
                matrix[0][j] = 0;
            }
        }
        if (first_col_zero) {
            for (int i = 0; i < m; i++) {
                matrix[i][0] = 0;
            }
        }
        return matrix;
    }
};
