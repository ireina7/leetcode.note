/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */
class Solution {
public:
    ListNode* mergeTwoLists(ListNode* l1, ListNode* l2) {
        if(l1 == NULL) return l2;
        if(l2 == NULL) return l1;

        ListNode *p1 = l1, *p2 = l2;
        ListNode newNode;
        ListNode *node = &newNode;
        while(p1 && p2) {
            ListNode *nextNode = new ListNode();
            int a = p1->val, b = p2->val;
            nextNode->val = a <= b ? (p1 = p1->next, a) : (p2 = p2->next, b);
            node->next = nextNode;
            node = nextNode;
        }
        if(p1) node->next = p1;
        if(p2) node->next = p2;

        return newNode.next;
    }
};
