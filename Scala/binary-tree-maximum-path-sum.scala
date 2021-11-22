/**
 * Definition for a binary tree node.
 * class TreeNode(_value: Int = 0, _left: TreeNode = null, _right: TreeNode = null) {
 *   var value: Int = _value
 *   var left: TreeNode = _left
 *   var right: TreeNode = _right
 * }
 */
object Solution {
    def maxPathSum(root: TreeNode): Int = {
        import scala.collection.mutable.Map
        val memo: Map[TreeNode, (Int, Int)] = Map()
        def dfs(root: TreeNode): (Int, Int) = {
            if (root == null) return (0, 0)
            if (memo.contains(root)) {
                return memo(root)
            }
            val (ll, lr) = dfs(root.left)
            val (rl, rr) = dfs(root.right)
            val lMax = ll max lr max 0
            val rMax = rl max rr max 0
            val res = (
                lMax + (if (root.left  != null) root.left .value else 0), 
                rMax + (if (root.right != null) root.right.value else 0),
            )
            memo += (root -> (lMax, rMax))
            res
        }
        if (root == null) {
            return Int.MinValue
        }
        val memo0: Map[TreeNode, Int] = Map()
        if (memo0.contains(root)) {
            return memo0(root)
        }
        val (l, r)  = dfs(root)
        //println(s"$l, $r")
        val curr = (l max 0) + root.value + (r max 0)
        val left  = maxPathSum(root.left)
        val right = maxPathSum(root.right)
        val res = curr max left max right
        memo0 += (root -> res)
        res
    }
}