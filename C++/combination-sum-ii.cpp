class Solution {
private:
    void search (
        vector<int>& candidates,
        int idx,
        vector<int> cur,
        int target,
        vector<vector<int>> &ans
    ) {
        if(target == 0) {
            ans.push_back(cur);
            return;
        }
        int pre(0);
        for(int i(idx); i < candidates.size();) {
            pre = candidates[i];
            if(pre <= target) {
                cur.push_back(pre);
                search(candidates, i + 1, cur, target - pre, ans);
                cur.pop_back();
            }
            // Handle duplicates
            while(i < candidates.size() && candidates[i] == pre) ++i;
        }
    }

public:
    vector<vector<int>> combinationSum2 (vector<int>& candidates, int target) {

        sort(candidates.begin(), candidates.end());
        vector<vector<int>> ans {};
        search(candidates, 0, {}, target, ans);
        return ans;
    }
};
