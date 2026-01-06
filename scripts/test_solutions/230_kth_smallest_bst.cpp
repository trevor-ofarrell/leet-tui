class Solution {
public:
    int kthSmallest(TreeNode* root, int k) {
        int result = 0;
        int count = 0;
        inorder(root, k, count, result);
        return result;
    }

private:
    void inorder(TreeNode* node, int k, int& count, int& result) {
        if (!node) return;
        inorder(node->left, k, count, result);
        count++;
        if (count == k) {
            result = node->val;
            return;
        }
        inorder(node->right, k, count, result);
    }
};
