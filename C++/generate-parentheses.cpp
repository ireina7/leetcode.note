class Solution {
public:
    set<string> generate(int n) {
        if(n == 1) return {"()"};
        set<string> ans;
        for(int i = 1; i < n; ++i) {
            auto s0 = generate(i);
            auto s1 = generate(n - i);
            for(auto &a : s0) {
                for(auto &b : s1) {
                    ans.insert(a + b);
                    if(i == 1) ans.insert("(" + b + ")");
                }
            }
        }
        return ans;
    }
    set<string> generate_dp(int n) {
        vector<set<string>> rc(n + 1);
        rc[1] = {"()"};
        for(int i = 2; i <= n; ++i) {
            for(int j = 1; j < n / 2 + 1; ++j) {
                for(auto &a : rc[j]) {
                    for(auto &b : rc[i - j]) {
                        rc[i].insert(a + b);
                        rc[i].insert(b + a);
                        if(j == 1) rc[i].insert("(" + b + ")");
                    }
                }
            }
        }
        return rc[n];
    }
    vector<string> generateParenthesis(int n) {
        auto s = generate_dp(n);
        return vector<string>(s.begin(), s.end());
    }
};
