class Solution {
public:
    int search(vector<int>& nums, int target) {

        int i{0}, j(nums.size() - 1);
        while(i < nums.size() && j >= 0 && i <= j) {
            int k = (i + j) / 2;
            if(nums[k] == target) {
                return k;
            }
            if(nums[k] >= nums[i]) {
                if(nums[k] < target) {
                    i = k + 1;
                }
                else {
                    if(nums[i] < target) {
                        j = k - 1;
                    }
                    else if(nums[i] == target) {
                        return i;
                    }
                    else {
                        i = k + 1;
                    }
                }
            }
            else {
                if(nums[k] > target) {
                    j = k - 1;
                }
                else {
                    if(nums[j] == target) {
                        return j;
                    }
                    else if(nums[j] > target) {
                        i = k + 1;
                    }
                    else {
                        j = k - 1;
                    }
                }
            }
        }
        return -1;
    }
};
