class Solution {
public:
    int romanToInt(string s) {
        unordered_map<char, int> m {
            {'O',    0},
            {'I',    1},
            {'V',    5},
            {'X',   10},
            {'L',   50},
            {'C',  100},
            {'D',  500},
            {'M', 1000},
        };
        int sum = 0;
        bool flag = false;
        char pre = 'O';
        for(auto it = s.crbegin(); it != s.crend(); ++it) {
            const char &x = *it;
            if(flag && m[x] < m[pre]) {
                switch(x) {
                    case 'I':
                    case 'X':
                    case 'C':
                        sum -= m[x];
                        break;
                    default:
                        sum += m[x];
                }
                flag = false;
            }
            else {
                sum += m[x];
            }
            flag =
                x == 'M' || x == 'D' || x == 'C' ||
                x == 'L' || x == 'X' || x == 'V';
            pre = x;
        }
        return sum;
    }
};
