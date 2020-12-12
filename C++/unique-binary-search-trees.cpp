class Solution {
public:
    int numTrees(int n) {
        /*
        for(int i(0); i <= n - 1; ++i) {
            sum += numTrees(i) * numTrees(n - i - 1);
        }
        */
        vector<long> rc(n + 1, 0);
        rc[0] = 1, rc[1] = 1;
        for(int i(2); i <= n; ++i)
            for(int j(0); j < i; ++j)
                rc[i] += rc[j] * rc[i - j - 1];

        return rc[n];
    }
};
