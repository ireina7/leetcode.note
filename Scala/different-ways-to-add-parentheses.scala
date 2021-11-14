object Solution {
    def diffWaysToCompute(expr: String): List[Int] = {
        import scala.collection.mutable.Map
        val ops = Map(
            '+' -> ((a: Int, b: Int) => a + b),
            '-' -> ((a: Int, b: Int) => a - b),
            '*' -> ((a: Int, b: Int) => a * b),
        )
        val memo: Map[String, List[Int]] = Map()
        def dfs(expr: String): List[Int] = {
            if (expr forall Character.isDigit) {
                return List(expr.toInt)
            }
            if (memo.contains(expr)) {
                return memo(expr)
            }
            val res = for {
                i <- (0 until expr.length).toList
                if ops isDefinedAt expr(i)
                x <- dfs(expr.slice(0, i))
                y <- dfs(expr.slice(i + 1, expr.length))
            } yield {
                ops(expr(i))(x, y)
            }
            memo += (expr -> res)
            res
        }
        dfs(expr)
    }
}