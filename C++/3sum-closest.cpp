class Solution {
public:
    int threeSumClosest(vector<int>& nums, int t) {
        int n = nums.size();
        if(n < 3) return 0;

        int ans(nums[0] + nums[1] + nums[2]);
        sort(nums.begin(), nums.end());
        for(int k(0); k < n - 2; ++k) {
            int target = t - nums[k];
            for(int i(k + 1), j(n - 1); i < j;) {

                int c = nums[i] + nums[j];
                if(c == target) {
                    return t;
                }
                else if(c < target) {
                    if(abs(t - ans) > t - (c + nums[k])) {
                        ans = c + nums[k];
                    }
                    i += 1;
                }
                else {
                    if(abs(t - ans) > (c + nums[k]) - t) {
                        ans = c + nums[k];
                    }
                    j -= 1;
                }
            }
            while(k < n - 2 && nums[k + 1] == nums[k]) ++k;
        }
        return ans;
    }
};
