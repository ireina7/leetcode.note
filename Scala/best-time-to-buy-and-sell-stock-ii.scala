object Solution {
    def maxProfit(prices: Array[Int]): Int = {
        import scala.collection.mutable.Map
        val memo: Map[(Int, Boolean), Int] = Map()
        def dfs(i: Int, buying: Boolean): Int = {
            if (i == 0 && buying == false) return 0
            if (i == 0 && buying == true ) return -prices(0)
            if (memo.contains((i, buying))) {
                return memo((i, buying))
            }
            val res = buying match {
                case false => 
                    dfs(i - 1, false) max (dfs(i - 1, true ) + prices(i))
                case true  => 
                    dfs(i - 1, true ) max (dfs(i - 1, false) - prices(i))
            }
            memo += ((i, buying) -> res)
            res
        }
        dfs(prices.length - 1, false)
    }
}