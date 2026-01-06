class Solution {
public:
    vector<vector<string>> solveNQueens(int n) {
        vector<vector<string>> result;
        vector<string> board(n, string(n, '.'));
        backtrack(result, board, 0, n);
        return result;
    }

private:
    bool isValid(vector<string>& board, int row, int col, int n) {
        for (int i = 0; i < row; i++) {
            if (board[i][col] == 'Q') return false;
        }
        int i = row - 1, j = col - 1;
        while (i >= 0 && j >= 0) {
            if (board[i][j] == 'Q') return false;
            i--;
            j--;
        }
        i = row - 1;
        j = col + 1;
        while (i >= 0 && j < n) {
            if (board[i][j] == 'Q') return false;
            i--;
            j++;
        }
        return true;
    }

    void backtrack(vector<vector<string>>& result, vector<string>& board, int row, int n) {
        if (row == n) {
            result.push_back(board);
            return;
        }
        for (int col = 0; col < n; col++) {
            if (isValid(board, row, col, n)) {
                board[row][col] = 'Q';
                backtrack(result, board, row + 1, n);
                board[row][col] = '.';
            }
        }
    }
};
