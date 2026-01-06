class Solution {
public:
    int goodNodes(TreeNode* root) {
        return dfs(root, INT_MIN);
    }

private:
    int dfs(TreeNode* node, int maxVal) {
        if (!node) return 0;

        int count = 0;
        if (node->val >= maxVal) {
            count = 1;
        }

        maxVal = max(maxVal, node->val);
        count += dfs(node->left, maxVal);
        count += dfs(node->right, maxVal);

        return count;
    }
};
