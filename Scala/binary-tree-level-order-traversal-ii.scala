/**
 * Definition for a binary tree node.
 * class TreeNode(_value: Int = 0, _left: TreeNode = null, _right: TreeNode = null) {
 *   var value: Int = _value
 *   var left: TreeNode = _left
 *   var right: TreeNode = _right
 * }
 */
object Solution {
    def levelOrderBottom(root: TreeNode): List[List[Int]] = {
        dfs(root).reverse
    }
    def dfs(root: TreeNode): List[List[Int]] = {
        if (root == null) {
            return Nil
        }
        val lefts  = dfs(root.left)
        val rights = dfs(root.right)
        val merge = {
            var i = 0
            var j = 0
            var xs: List[List[Int]] = List()
            while (i < lefts.length || j < rights.length) {
                val x = if (i >= lefts .length) Nil else lefts(i)
                val y = if (i >= rights.length) Nil else rights(i)
                xs = xs :+ (x ::: y)
                i += 1
                j += 1
            }
            xs
        }
        List(root.value) :: merge
    }
}