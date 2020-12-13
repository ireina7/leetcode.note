class Solution {
public:
    vector<vector<string>> groupAnagrams(vector<string>& strs) {

        unordered_map<string, vector<string>> map;
        vector<vector<string>> ans {};
        for(const string &s : strs) {
            string ss = s;
            sort(ss.begin(), ss.end());
            map[ss].push_back(s);
        }
        for(auto &p : map) {
            ans.push_back(move(p.second));
        }
        return ans;
    }
};
