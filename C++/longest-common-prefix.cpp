class Solution {
public:
    string longestCommonPrefix(vector<string>& strs) {
        if(strs.empty()) return "";

        string ans = "";
        for(int i = 0;; ++i) {
            char c = '\0';
            for(string s : strs) {
                if(i == s.size()) return ans;
                if(c == '\0') {
                    c = s[i];
                    continue;
                }
                if(s[i] != c) return ans;
            }
            ans.push_back(c);
        }
        return ans;
    }
};
