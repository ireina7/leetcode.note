class Solution {
public:
    bool isPalindrome(int x) {
        string xs = to_string(x);
        string sx = xs;
        reverse(sx.begin(), sx.end());
        return xs == sx;
    }
};
