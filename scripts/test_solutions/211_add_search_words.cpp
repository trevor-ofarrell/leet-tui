class WordDictionary {
private:
    struct TrieNode {
        unordered_map<char, TrieNode*> children;
        bool isEnd = false;
    };
    TrieNode* root;

    bool dfs(TrieNode* node, const string& word, int i) {
        if (i == word.size()) {
            return node->isEnd;
        }
        if (word[i] == '.') {
            for (auto& p : node->children) {
                if (dfs(p.second, word, i + 1)) {
                    return true;
                }
            }
            return false;
        }
        if (node->children.find(word[i]) == node->children.end()) {
            return false;
        }
        return dfs(node->children[word[i]], word, i + 1);
    }

public:
    WordDictionary() {
        root = new TrieNode();
    }

    void addWord(string word) {
        TrieNode* node = root;
        for (char c : word) {
            if (node->children.find(c) == node->children.end()) {
                node->children[c] = new TrieNode();
            }
            node = node->children[c];
        }
        node->isEnd = true;
    }

    bool search(string word) {
        return dfs(root, word, 0);
    }
};
