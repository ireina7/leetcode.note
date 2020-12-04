class Solution {
public:
    void do_loop(vector<vector<int>> &matrix, int n, int i, int j) {

        int x = j - i;
        int a = matrix[i][j];
        int b = matrix[i + x][j - x + n - 1];
        int c = matrix[i + n - 1][j - x + n - 1 - x];
        int d = matrix[i + n - 1 - x][j - x];
        matrix[i][j] = d;
        matrix[i + x][j - x + n - 1] = a;
        matrix[i + n - 1][j - x + n - 1 - x] = b;
        matrix[i + n - 1 - x][j - x] = c;
    }
    void do_circle(vector<vector<int>> &matrix, int idx) {

        int n = matrix.size() - idx * 2;
        for(int i(idx); i < (idx + n - 1); ++i) {
            do_loop(matrix, n, idx, i);
        }
    }
    void rotate(vector<vector<int>>& matrix) {
        if(matrix.empty()) return;

        int circles = matrix[0].size() / 2;
        for(int i(0); i < circles; ++i) {
            do_circle(matrix, i);
        }
    }
};
