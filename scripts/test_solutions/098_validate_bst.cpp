class Solution {
public:
    bool isValidBST(TreeNode* root) {
        return validate(root, LLONG_MIN, LLONG_MAX);
    }

private:
    bool validate(TreeNode* node, long long minVal, long long maxVal) {
        if (!node) {
            return true;
        }
        if (node->val <= minVal || node->val >= maxVal) {
            return false;
        }
        return validate(node->left, minVal, node->val) &&
               validate(node->right, node->val, maxVal);
    }
};
