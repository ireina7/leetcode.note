class Solution {
public:
    bool isChar(char c){
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z');
    }
    bool isdigit(char c){
        return (c >= '0' && c <= '9');
    }
    int myAtoi(string s) {

        long long int ans = 0;
        int n = s.length();
        if(n == 0) return 0;
        bool neg = false;
        int i = 0;
        while(i < n && s[i] == ' ') i++;
        if(s[i] == '-') neg = true, i++;
        else if(s[i] == '+') neg = false, i++;

        if(isChar(s[i])) return 0;
        for(; i < n && isdigit(s[i]); ++i) {
            ans = ans * 10 + (s[i] - '0');
            if(neg && ans * (-1) < INT_MIN) return INT_MIN;
            if(!neg && ans > INT_MAX) return INT_MAX;
        }
        return (neg == false) ? ans : -ans;
    }
};
