/**
 * Definition for a Node.
 * class Node(var _value: Int) {
 *   var value: Int = _value
 *   var left: Node = null
 *   var right: Node = null
 *   var next: Node = null
 * }
 */

object Solution {
    def connect(root: Node): Node = {
        if(root == null || root.left == null || root.left == null) return root

        root.left.next = root.right
        root.right.next = if(root.next == null) null else root.next.left
        connect(root.left)
        connect(root.right)
        root
    }
}
