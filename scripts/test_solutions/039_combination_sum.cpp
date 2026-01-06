class Solution {
public:
    vector<vector<int>> combinationSum(vector<int>& candidates, int target) {
        vector<vector<int>> result;
        vector<int> path;
        backtrack(candidates, target, 0, path, result);
        return result;
    }

private:
    void backtrack(vector<int>& candidates, int remaining, int start,
                   vector<int>& path, vector<vector<int>>& result) {
        if (remaining == 0) {
            result.push_back(path);
            return;
        }
        if (remaining < 0) {
            return;
        }
        for (int i = start; i < candidates.size(); i++) {
            path.push_back(candidates[i]);
            backtrack(candidates, remaining - candidates[i], i, path, result);
            path.pop_back();
        }
    }
};
