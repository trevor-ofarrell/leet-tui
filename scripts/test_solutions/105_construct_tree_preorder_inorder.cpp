class Solution {
public:
    TreeNode* buildTree(vector<int>& preorder, vector<int>& inorder) {
        unordered_map<int, int> idx_map;
        for (int i = 0; i < inorder.size(); i++) {
            idx_map[inorder[i]] = i;
        }
        int pre_idx = 0;
        return build(preorder, idx_map, pre_idx, 0, inorder.size() - 1);
    }

private:
    TreeNode* build(vector<int>& preorder, unordered_map<int, int>& idx_map,
                    int& pre_idx, int left, int right) {
        if (left > right) return nullptr;
        int val = preorder[pre_idx++];
        TreeNode* node = new TreeNode(val);
        int idx = idx_map[val];
        node->left = build(preorder, idx_map, pre_idx, left, idx - 1);
        node->right = build(preorder, idx_map, pre_idx, idx + 1, right);
        return node;
    }
};
