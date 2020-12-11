class Solution {
public:
    int maxSubArray(vector<int>& nums) {
        if(nums.empty()) return 0;

        int ans{nums[0]}, cur{ans};
        for(int i{1}; i < nums.size(); ++i) {
            cur = max(nums[i], nums[i] + cur);
            ans = max(ans, cur);
        }
        return ans;
    }
};
