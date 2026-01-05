var Trie = function() { this.root = {}; };
Trie.prototype.insert = function(word) {
    let node = this.root;
    for (const c of word) { if (!node[c]) node[c] = {}; node = node[c]; }
    node.isEnd = true;
};
Trie.prototype.search = function(word) {
    let node = this.root;
    for (const c of word) { if (!node[c]) return false; node = node[c]; }
    return node.isEnd === true;
};
Trie.prototype.startsWith = function(prefix) {
    let node = this.root;
    for (const c of prefix) { if (!node[c]) return false; node = node[c]; }
    return true;
};
