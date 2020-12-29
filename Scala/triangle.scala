object Solution {

    def minimumTotal(triangle: List[List[Int]]): Int = {
        if(triangle.isEmpty) return 0

        val memo = scala.collection.mutable.Map[(Int, Int), Int]()
        def select(i: Int, j: Int): Int = (i, j) match {
            case (i, j) if(i + 1 == triangle.length) => triangle(i)(j)
            case (i, j) => memo.getOrElse((i, j), {
                val ans = triangle(i)(j) + (select(i+1, j) min select(i+1, j+1))
                memo += ((i, j) -> ans)
                ans
            })
        }
        return select(0, 0)
    }
}
