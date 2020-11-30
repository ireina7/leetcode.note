class Solution {
public:
    int maxArea(vector<int>& h) {
        int area = 0, i = 0, j = h.size() - 1;
        while (i < j) {
            area = max(area, min(h[i], h[j]) * (j - i));
            h[i] < h[j] ? ++i : --j;
        }
        return area;
    }
};
