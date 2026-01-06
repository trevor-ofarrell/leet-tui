class Codec {
public:
    string serialize(TreeNode* root) {
        string result;
        dfsSerialize(root, result);
        return result;
    }

    TreeNode* deserialize(string data) {
        int idx = 0;
        return dfsDeserialize(data, idx);
    }

private:
    void dfsSerialize(TreeNode* node, string& result) {
        if (!node) {
            result += "null,";
        } else {
            result += to_string(node->val) + ",";
            dfsSerialize(node->left, result);
            dfsSerialize(node->right, result);
        }
    }

    TreeNode* dfsDeserialize(const string& data, int& idx) {
        int start = idx;
        while (idx < data.length() && data[idx] != ',') {
            idx++;
        }
        string val = data.substr(start, idx - start);
        idx++;  // skip comma

        if (val == "null") {
            return nullptr;
        }
        TreeNode* node = new TreeNode(stoi(val));
        node->left = dfsDeserialize(data, idx);
        node->right = dfsDeserialize(data, idx);
        return node;
    }
};
