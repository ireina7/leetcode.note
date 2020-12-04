class Solution {
public:
    vector<vector<int>> fourSum(vector<int>& nums, int t) {
        int n = nums.size();
        if(n < 4) return {};

        vector<vector<int>> ans {};
        sort(nums.begin(), nums.end());
        for(int h(0); h < n - 3; ++h) {
            for(int k(h + 1); k < n - 2; ++k) {
                int target = t - nums[k] - nums[h];
                for(int i(k + 1), j(n - 1); i < j;) {

                    int c = nums[i] + nums[j];
                    if(c == target) {
                        ans.push_back({nums[h], nums[k], nums[i], nums[j]});
                        while(i < j && nums[i + 1] == nums[i]) ++i;
                        while(i < j && nums[j - 1] == nums[j]) --j;
                        ++i, --j;
                    }
                    else if(c < target) i += 1;
                    else j -= 1;
                }
                while(k < n - 2 && nums[k + 1] == nums[k]) ++k;
            }
            while(h < n - 3 && nums[h + 1] == nums[h]) ++h;
        }
        return ans;
    }
};
