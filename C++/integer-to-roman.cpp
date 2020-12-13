class Solution {
public:
    string intToRoman(int num) {
        map<int, string> m = {
            //{   0, "O" },
            {   1, "I" },
            {   4, "IV"},
            {   5, "V" },
            {   9, "IX"},
            {  10, "X" },
            {  40, "XL"},
            {  50, "L" },
            {  90, "XC"},
            { 100, "C" },
            { 400, "CD"},
            { 500, "D" },
            { 900, "CM"},
            {1000, "M" },
        };
        string ans {""};
        auto it = m.begin();
        for(auto iter = m.begin(); iter != m.end(); ++iter) {
            if(num / iter->first != 0) it = iter;
        }
        for(int n = num; it != m.end(); --it) {
            while(n / (it->first)) {
                ans += it->second;
                n -= it->first;
            }
        }
        return ans;
    }
};
