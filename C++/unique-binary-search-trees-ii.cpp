/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
 * };
 */
class Solution {
public:
    vector<TreeNode*> gen(const vector<int> &ns) {
        if(ns.empty()) return {nullptr};

        vector<TreeNode*> ans {};
        for(int i(0); i <= ns.size() - 1; ++i) {

            vector<int> ls = ns, rs = ns;
            ls.erase(ls.begin() + i, ls.end());
            rs.erase(rs.begin(), rs.begin() + i + 1);
            auto as  = gen(ls), bs = gen(rs);
            for(auto a : as) {
                for(auto b : bs) {
                    TreeNode *node = new TreeNode(ns[i]);
                    node->left = a, node->right = b;
                    ans.push_back(node);
                }
            }
        }
        return ans;
    }
    vector<TreeNode*> generateTrees(int n) {
        if(n == 0) return {};

        vector<int> ns {};
        for(int i(1); i <= n; ++i) {
            ns.push_back(i);
        }
        return gen(ns);
    }
};
