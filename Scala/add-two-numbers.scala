/**
 * Definition for singly-linked list.
 * class ListNode(_x: Int = 0, _next: ListNode = null) {
 *   var next: ListNode = _next
 *   var x: Int = _x
 * }
 */
object Solution {
    /**
     * This implementation is ugly, however, I don't want to spend time on improving it 
     */
    def addTwoNumbers(l1: ListNode, l2: ListNode): ListNode = {
        def add(a: Int, b: Int, c: Int) = {
            val sum = a + b + c
            (sum % 10, sum / 10)
        }
        var (h1, h2) = (l1, l2)
        var c: Int = 0
        var ans = new ListNode(0)
        var cur = ans
        while(h1 != null && h2 != null) {
            val (x, inc) = add(h1.x, h2.x, c)
            c = inc
            cur.next = new ListNode(x)
            cur = cur.next
            h1 = h1.next
            h2 = h2.next
        }
        while(h1 != null) {
            val (x, inc) = add(h1.x, 0, c)
            c = inc
            cur.next = new ListNode(x)
            cur = cur.next
            h1 = h1.next
        }
        while(h2 != null) {
            val (x, inc) = add(0, h2.x, c)
            c = inc
            cur.next = new ListNode(x)
            cur = cur.next
            h2 = h2.next
        }
        if(c > 0) {
            cur.next = new ListNode(c)
        }
        ans.next
    }
}