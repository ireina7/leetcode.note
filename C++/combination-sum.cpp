class Solution {
    void combinationSumUtil(
        vector<int> &candidates, int target, int l,
        vector<int> &v,
        vector<vector<int>> &ans
    ) {
        if (target == 0) {
            ans.push_back(v);
            return;
        }
        for (int i = l; i < candidates.size(); i++) {
            if (candidates[i] <= target) {
                v.push_back(candidates[i]);
                combinationSumUtil(candidates, target-candidates[i], i, v, ans);
                v.pop_back();
            }
        }
    }

public:
    vector<vector<int>> combinationSum(vector<int>& candidates, int target) {

        vector<vector<int>> ans {};
        vector<int> v {};
        combinationSumUtil(candidates, target, 0, v, ans);
        return ans;
    }
};
