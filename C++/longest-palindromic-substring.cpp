/**
   Manacher's Algorithm
 */
class Solution {
public:
    string longestPalindrome(string s) {
        string t ="$#";
        for (int i = 0; i < s.size(); ++i) {
            t += s[i];
            t += '#';
        }
        vector<int> p(t.size(), 0);
        int id = 0, mx = 0, resId = 0, resMx = 0;
        for (int i = 1; i < t.size(); ++i) {
            p[i] = mx > i ? min(p[2 * id - i], mx - i) : 1;
            while (t[i + p[i]] == t[i - p[i]]) ++p[i];
            if (mx < i + p[i]) {
                mx = i + p[i];
                id = i;
            }
            if (resMx < p[i]) {
                resMx = p[i];
                resId = i;
            }
        }
        return s.substr((resId - resMx) / 2, resMx - 1);
    }
};


/**
   DP solution
 */
class Solution {
public:
    string longestPalindrome(string s) {

        vector<vector<int>> rc (
            s.size() + 1, vector<int>(s.size(), 0)
        );
        int start = 0, k = 0;
        for(int n(1); n <= s.size(); ++n) {
            for(int i(0); i <= s.size() - n; ++i) {
                rc[n][i] =
                    n == 1 ? 1 :
                    n == 2 ? (s[i] == s[i + 1] ? 2 : 0) :
                    rc[n - 2][i + 1] ? (s[i] == s[i + n - 1] ? rc[n - 2][i + 1] + 2 : 0) : 0;

                if(rc[n][i] > k) {
                    start = i;
                    k = n;
                }
            }
        }
        return s.substr(start, k);
    }
};
