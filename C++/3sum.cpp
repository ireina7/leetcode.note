class Solution {
public:
    vector<vector<int>> threeSum(vector<int>& nums) {
        int n = nums.size();
        if(n < 3) return {};

        vector<vector<int>> ans {};
        sort(nums.begin(), nums.end());
        if(nums[0] > 0 || nums[n - 1] < 0) return {};
        for(int k(0); k < n - 2; ++k) {
            int target = 0 - nums[k];
            for(int i(k + 1), j(n - 1); i < j;) {

                int c = nums[i] + nums[j];
                if(c == target) {
                    ans.push_back({nums[k], nums[i], nums[j]});
                    while(i < j && nums[i + 1] == nums[i]) ++i;
                    while(i < j && nums[j - 1] == nums[j]) --j;
                    ++i, --j;
                }
                else if(c < target) i += 1;
                else j -= 1;
            }
            while(k < n - 2 && nums[k + 1] == nums[k]) ++k;
        }
        return ans;
    }
};
