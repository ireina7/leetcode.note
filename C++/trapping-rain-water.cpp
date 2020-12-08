class Solution {
public:
    int onePass(vector<int> &hs) {
        if(hs.empty()) return 0;

        int ans{0}, acc{0}, curMax{hs[0]};
        for(int i{1}; i < hs.size(); ++i) {

            if(hs[i] < curMax) {
                acc += curMax - hs[i];
            }
            else {
                ans = ans + acc;
                acc = 0;
                curMax = hs[i];
            }
        }
        return ans;
    }
    int findMax(vector<int>& hs) {
        int ans = 0;
        for(int i(0); i < hs.size(); ++i) {
            if(hs[ans] < hs[i]) ans = i;
        }
        return ans;
    }
    int trap(vector<int>& height) {
        if(height.empty()) return 0;

        int i = findMax(height);
        height[i] += 1;
        int a = onePass(height);
        reverse(height.begin(), height.end());
        int b = onePass(height);

        return a + b;
    }
};
