class Solution {
public:
    bool isValid(string s) {
        stack<char> stk;
        for(char &c : s) {
            if(stk.empty()) {
                if(c == ')' || c == ']' || c == '}') return false;
                stk.push(c);
                continue;
            }
            if(c == '(' || c == '[' || c == '{') {
                stk.push(c);
                continue;
            }
            if(c == ')') {
                if(stk.top() != '(') return false;
                else stk.pop();
                continue;
            }
            if(c == ']') {
                if(stk.top() != '[') return false;
                else stk.pop();
                continue;
            }
            if(c == '}') {
                if(stk.top() != '{') return false;
                else stk.pop();
                continue;
            }
        }
        return stk.empty();
    }
};
