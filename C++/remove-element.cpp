class Solution {
public:
    int removeElement(vector<int>& nums, int val) {

        int ans = 0, n = nums.size();
        for(int i(0), j(0); i < n; ++i) {
            int k = i;
            while(k > j) nums[k-1] = nums[k], --k;
            ans += (nums[k] == val) ? 1 : (j = k + 1, 0);
        }
        return n - ans;
    }
};
