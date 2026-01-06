class Solution {
private:
    struct TrieNode {
        unordered_map<char, TrieNode*> children;
        string word = "";
    };

    void dfs(vector<vector<char>>& board, int i, int j, TrieNode* node, vector<string>& result) {
        int m = board.size(), n = board[0].size();
        if (i < 0 || i >= m || j < 0 || j >= n) return;

        char c = board[i][j];
        if (c == '#' || node->children.find(c) == node->children.end()) return;

        node = node->children[c];
        if (!node->word.empty()) {
            result.push_back(node->word);
            node->word = "";
        }

        board[i][j] = '#';
        dfs(board, i + 1, j, node, result);
        dfs(board, i - 1, j, node, result);
        dfs(board, i, j + 1, node, result);
        dfs(board, i, j - 1, node, result);
        board[i][j] = c;
    }

public:
    vector<string> findWords(vector<vector<char>>& board, vector<string>& words) {
        TrieNode* root = new TrieNode();

        for (const string& w : words) {
            TrieNode* node = root;
            for (char c : w) {
                if (node->children.find(c) == node->children.end()) {
                    node->children[c] = new TrieNode();
                }
                node = node->children[c];
            }
            node->word = w;
        }

        vector<string> result;
        int m = board.size(), n = board[0].size();

        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                dfs(board, i, j, root, result);
            }
        }
        return result;
    }
};
