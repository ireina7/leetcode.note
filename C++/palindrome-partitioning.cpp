/**
   Naive recursion
*/
class Solution {
public:
    vector<vector<string>> partition(string s) {
        vector<vector<string>> ans;
        if(s.empty()) return {{}};
        for(int i = 0; i < s.size(); ++i) {
            if(!isPalindrome(s, i + 1)) continue;
            for(auto list : partition(s.substr(i + 1))) {
                list.insert(list.begin(), s.substr(0, i + 1));
                ans.push_back(list);
            }
        }
        return ans;
    }
    bool isPalindrome(string s, int n) {
        for(int i = 0; i < n / 2; ++i) {
            if(s[i] != s[n - 1 - i]) return false;
        }
        return true;
    }
};


/**
   Dynamic programming
 */
