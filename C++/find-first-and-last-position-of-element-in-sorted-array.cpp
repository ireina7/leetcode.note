class Solution {
public:
    vector<int> radar(vector<int> &nums, const int k) {
        int i(k - 1), j(k + 1);
        while(i >= 0 && i < nums.size() && nums[i] == nums[k]) --i;
        while(j >= 0 && j < nums.size() && nums[j] == nums[k]) ++j;
        ++i, --j;
        return {i, j};
    }

    vector<int> searchRange(vector<int>& nums, int target) {
        if(nums.empty()) return {-1, -1};

        int i{0}, j(nums.size() - 1);
        while(i <= j) {

            int k = (i + j) / 2;
            if(nums[k] == target) {

                return radar(nums, k);
            }
            else if(nums[k] < target) {
                i = k + 1;
            }
            else {
                j = k - 1;
            }
        }
        return {-1, -1};
    }
};
