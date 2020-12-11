class Solution {
public:
    vector<int> spiralOrder(vector<vector<int>>& matrix) {
        if(matrix.empty()) return {};

        vector<int> ans {};
        int cl {0}, ru {0};
        int cr (matrix[0].size() - 1), rd (matrix.size() - 1);

        while(cl <= cr && ru <= rd) {

            for(int i(cl); i <= cr; i++) ans.push_back(matrix[ru][i]);
            ru++;

            if(ru > rd) break;

            for(int i(ru); i <= rd; i++) ans.push_back(matrix[i][cr]);
            cr--;

            if(cl > cr) break;

            for(int i(cr); i >= cl; i--) ans.push_back(matrix[rd][i]);
            rd--;

            if(ru > rd) break;

            for(int i(rd); i >= ru; i--) ans.push_back(matrix[i][cl]);
            cl++;
        }
        return ans;
    }
};
