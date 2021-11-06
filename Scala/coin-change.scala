object Solution {
    def coinChange(coins: Array[Int], amount: Int): Int = {
        import scala.collection.mutable.HashMap
        val memo = HashMap.empty[Int, Option[Int]]
        def cc(coins: List[Int], amount: Int): Option[Int] = {
            if (memo.contains(amount)) memo(amount)
            else {
              val res = if (amount < 0) None
              else if (amount == 0) Some(0)
              else coins.flatMap(c => cc(coins, amount - c).map(_ + 1)).minOption

              memo.put(amount, res)
              res
            }
        }
        cc(coins.toList, amount).getOrElse(-1)
    }
}