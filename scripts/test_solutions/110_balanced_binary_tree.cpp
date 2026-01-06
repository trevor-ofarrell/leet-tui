class Solution {
public:
    bool isBalanced(TreeNode* root) {
        return height(root) != -1;
    }

private:
    int height(TreeNode* node) {
        if (!node) return 0;
        int left = height(node->left);
        int right = height(node->right);
        if (left == -1 || right == -1 || abs(left - right) > 1) {
            return -1;
        }
        return 1 + max(left, right);
    }
};
