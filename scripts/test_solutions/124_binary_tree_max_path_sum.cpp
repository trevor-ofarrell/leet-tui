class Solution {
public:
    int maxPathSum(TreeNode* root) {
        int max_sum = INT_MIN;
        dfs(root, max_sum);
        return max_sum;
    }

private:
    int dfs(TreeNode* node, int& max_sum) {
        if (!node) return 0;
        int left = max(0, dfs(node->left, max_sum));
        int right = max(0, dfs(node->right, max_sum));
        max_sum = max(max_sum, node->val + left + right);
        return node->val + max(left, right);
    }
};
