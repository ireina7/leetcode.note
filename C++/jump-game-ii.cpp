class Solution {
public:
    int jump(vector<int>& nums) {

        int ans = 0, n = nums.size(), last = 0, cur = 0;
        for(int i = 0; i < n - 1; ++i) {
            cur = max(cur, i + nums[i]);
            if(i == last) {
                last = cur;
                ++ans;
                if(cur >= n - 1) break;
            }
        }
        return ans;
    }
};
