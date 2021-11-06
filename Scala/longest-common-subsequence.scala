object Solution {
  def longestCommonSubsequence(text1: String, text2: String): Int = {
    var memo: Map[(Int, Int), Int] = Map()
    def dfs(i: Int, j: Int): Int = {
      if (memo.contains((i, j))) {
        return memo((i, j))
      }
      if (i >= text1.length || j >= text2.length) {
        return 0
      }
      val result = 
        if (text1(i) == text2(j)) dfs(i + 1, j + 1) + 1
        else dfs(i + 1, j) max dfs(i, j + 1)
      memo += ((i, j) -> result)
      result
    }
    val res = dfs(0, 0)
    res
  }
}