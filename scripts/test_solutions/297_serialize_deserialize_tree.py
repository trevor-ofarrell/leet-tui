class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Codec:
    def serialize(self, root):
        result = []

        def dfs(node):
            if not node:
                result.append('null')
            else:
                result.append(str(node.val))
                dfs(node.left)
                dfs(node.right)

        dfs(root)
        return ','.join(result)

    def deserialize(self, data):
        vals = iter(data.split(','))

        def dfs():
            val = next(vals)
            if val == 'null':
                return None
            node = TreeNode(int(val))
            node.left = dfs()
            node.right = dfs()
            return node

        return dfs()
