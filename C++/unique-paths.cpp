class Solution {
public:
    int walk(int i, int j, int m, int n) {
        return
            i == m && j == n ? 1 :
            i == m ? walk(i, j + 1, m, n) :
            j == n ? walk(i + 1, j, m, n) :
            /*else*/ walk(i + 1, j, m, n) + walk(i, j + 1, m, n);
    }
    int uniquePaths(int m, int n) {

        int rc[n];
        for(int i(m - 1); i >= 0; --i)
            for(int j(n - 1); j >= 0; --j)
                rc[j] =
                    i == m - 1 && j == n-1 ? 1     :
                    i == m - 1 ? rc[j + 1]         :
                    j == n - 1 ? rc[j]             :
                    /* else */   rc[j] + rc[j + 1] ;

        return rc[0];
    }
};
