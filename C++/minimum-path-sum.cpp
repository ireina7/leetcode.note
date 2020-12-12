class Solution {
public:
    int minPathSum(vector<vector<int>>& grid) {
        if(grid.empty()) return 0;

        const int M = grid.size(), N = grid[0].size();
        vector<int> rc(N, 0);
        for(int i(M - 1); i >= 0; --i)
            for(int j(N - 1); j >= 0; --j)
                rc[j] = grid[i][j] + (
                    i == M - 1 && j == N - 1 ? 0 :
                    i == M - 1 ? rc[j + 1]       :
                    j == N - 1 ? rc[j]           :
                    min(rc[j], rc[j + 1])
                );
        return rc[0];
    }
};
