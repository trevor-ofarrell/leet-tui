class WordDictionary:
    def __init__(self):
        self.root = {}

    def addWord(self, word):
        node = self.root
        for c in word:
            if c not in node:
                node[c] = {}
            node = node[c]
        node['$'] = True

    def search(self, word):
        def dfs(node, i):
            if i == len(word):
                return '$' in node
            if word[i] == '.':
                for key in node:
                    if key != '$' and dfs(node[key], i + 1):
                        return True
                return False
            if word[i] not in node:
                return False
            return dfs(node[word[i]], i + 1)

        return dfs(self.root, 0)
