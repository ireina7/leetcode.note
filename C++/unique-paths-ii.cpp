class Solution {
public:
    int uniquePathsWithObstacles(vector<vector<int>>& grid) {
        if(grid.empty() || grid[0][0] == 1) return 0;

        int m(grid.size()), n(grid[0].size());
        long rc[n];
        for(int i(m - 1); i >= 0; --i) {
            for(int j(n - 1); j >= 0; --j) {
                if(i == m - 1 && j == n - 1) {
                    rc[j] = grid[i][j] == 1 ? 0 : 1;
                    continue;
                }
                rc[j] =
                    i == m - 1 ? (abs(grid[i][j+1] - 1) * rc[j+1])                  :
                    j == n - 1 ? (abs(grid[i+1][j] - 1) * rc[j  ])                  :
                    abs(grid[i][j+1] - 1) * rc[j+1] + abs(grid[i+1][j] - 1) * rc[j] ;
            }
        }
        return rc[0];
    }
};
