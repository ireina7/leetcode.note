class Solution {
public:
    double findMedianSortedArrays(vector<int>& nums1, vector<int>& nums2) {\

        const int M = nums1.size();
        const int N = nums2.size();
        vector<int> str(M + N, 0);

        int k = 0, i = 0, j = 0, l = 0, m = 0;
        while(i < M && j < N) {
            str[k++] = nums1[i] < nums2[j] ? nums1[i++] : nums2[j++];
        }
        for(l = 0; i < M && l < M-i; l++) str[k+l] = nums1[i+l];
        for(l = 0; j < N && l < N-j; l++) str[k+l] = nums2[j+l];

        m = (M + N) / 2;
        return (M + N) % 2 == 0 ?
            static_cast<double>(str[m-1] + str[m]) / 2 :
            static_cast<double>(str[m]);
    }
};
