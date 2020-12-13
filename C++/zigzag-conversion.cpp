class Solution {
public:
    string convert(string s, int numRows) {
        if(numRows == 1) return s;

        string ans = "";
        const int N = numRows * 2 - 2;
        int i = 0;
        for(int n = 0; i < s.size() && n < numRows; ++n) {
            ans.push_back(s[i]);
            const int M = N - n * 2;
            const int I = i;
            while(i < s.size()) {
                i += M;
                if(M != 0 && i < s.size()) {
                    ans.push_back(s[i]);
                }
                i += N - M;
                if(N != M && i < s.size()) {
                    ans.push_back(s[i]);
                }
            }
            i = I + 1;
        }
        return ans;
    }
};
