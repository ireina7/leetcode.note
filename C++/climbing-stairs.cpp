class Solution {
public:
    int climbStairs(int n) {

        long a = 1, b = 0, c = a + b;
        for(int i(0); i <= n; ++i)
            c = a + b,
            b = a,
            a = c;
        return b;
    }
};
