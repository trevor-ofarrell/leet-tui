var WordDictionary = function() { this.root = {}; };
WordDictionary.prototype.addWord = function(word) {
    let node = this.root;
    for (const c of word) { if (!node[c]) node[c] = {}; node = node[c]; }
    node.isEnd = true;
};
WordDictionary.prototype.search = function(word) {
    function dfs(node, i) {
        if (i === word.length) return node.isEnd === true;
        if (word[i] === '.') {
            for (const key in node) { if (key !== 'isEnd' && dfs(node[key], i + 1)) return true; }
            return false;
        }
        return node[word[i]] ? dfs(node[word[i]], i + 1) : false;
    }
    return dfs(this.root, 0);
};
