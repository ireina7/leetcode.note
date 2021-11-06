object Solution {
    def maxUncrossedLines(nums1: Array[Int], nums2: Array[Int]): Int = {
        var memo: Map[(Int, Int), Int] = Map()
        def dfs(i: Int, j: Int): Int = {
            if (memo.contains((i, j))) {
                return memo((i, j))
            }
            if (i >= nums1.length || j >= nums2.length) {
                return 0
            }
            val result = 
                if (nums1(i) == nums2(j)) 1 + dfs(i + 1, j + 1)
                else dfs(i + 1, j) max dfs(i, j + 1)
            memo += ((i, j) -> result)
            result
        }
        dfs(0, 0)
    }
}