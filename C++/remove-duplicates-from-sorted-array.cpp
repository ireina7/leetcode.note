class Solution {
public:
    int removeDuplicates(vector<int>& nums) {
        int ans = 0, n = nums.size();
        for(int i(1), j(1); i < n; ++i) {
            int k = i;
            nums[j] = nums[k], k = j;
            ans += (nums[k] == nums[k-1]) ? 1 : (j = k + 1, 0);
        }
        return n - ans;
    }
};
