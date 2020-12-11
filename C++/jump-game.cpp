class Solution {
public:
    bool canJump(vector<int>& nums) {
        if(nums.empty()) return false;

        int n(nums.size()), pre(n - 1);
        for(int i(n - 1); i >= 0; --i) {
            if(i + nums[i] >= pre) pre = i;
        }
        return pre == 0;
    }
};
