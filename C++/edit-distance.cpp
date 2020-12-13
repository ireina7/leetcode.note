class Solution {
public:
    int minDis(string &w1, string &w2, int i, int j) {
        if(i == w1.size() && j == w2.size()) return 0;
        if(i == w1.size()) return w2.size() - j;
        if(j == w2.size()) return w1.size() - i;

        int ans = 0;
        if(w1[i] == w2[j]) {
            ans = minDis(w1, w2, i+1, j+1);
        }
        else {
            ans = 1 + min (
                minDis(w1, w2, i+1, j+1),
                min (minDis(w1, w2, i+1, j), minDis(w1, w2, i, j+1))
            );
        }
        return ans;
    }
    int minDistance(string word1, string word2) {
        const int N1 = word1.size(), N2 = word2.size();
        vector<vector<int>> rc (
            N1 + 1, vector<int>(N2 + 1, 0)
        );
        for(int i(N1); i >= 0; --i) {
            for(int j(N2); j >= 0; --j) {
                rc[i][j] =
                    i == N1 && j == N2   ? 0      :
                    i == N1              ? N2 - j :
                    j == N2              ? N1 - i :
                    word1[i] == word2[j] ?
                        rc[i + 1][j + 1] :
                        1 + min(rc[i + 1][j + 1], min(rc[i + 1][j], rc[i][j + 1]));
            }
        }
        return rc[0][0];
    }
};
