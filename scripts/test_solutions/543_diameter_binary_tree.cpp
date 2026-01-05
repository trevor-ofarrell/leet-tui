class Solution {
public:
    int diameterOfBinaryTree(TreeNode* root) {
        int diameter = 0;
        depth(root, diameter);
        return diameter;
    }

private:
    int depth(TreeNode* node, int& diameter) {
        if (!node) return 0;
        int left = depth(node->left, diameter);
        int right = depth(node->right, diameter);
        diameter = max(diameter, left + right);
        return 1 + max(left, right);
    }
};
