/**
 * Definition for a binary tree node.
 * class TreeNode(_value: Int = 0, _left: TreeNode = null, _right: TreeNode = null) {
 *   var value: Int = _value
 *   var left: TreeNode = _left
 *   var right: TreeNode = _right
 * }
 */
object Solution {
    def allPossibleFBT(n: Int): List[TreeNode] = {
        if (n == 1) List(new TreeNode())
        else for {
            i <- Range(1, n - 1, 2).toList
            l <- allPossibleFBT(i)
            r <- allPossibleFBT(n - 1 - i)
        } yield new TreeNode(0, l, r)
    }
}
