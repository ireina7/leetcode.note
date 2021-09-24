object Solution {
    def processQueries(queries: Array[Int], m: Int): Array[Int] = {
        var res = Array.empty[Int]
        queries.foldLeft((1 to m).toList) { case (ms, q) =>
            res :+= ms.indexOf(q)
            q :: ms.filter(_ != q)
        }
        res
    }
}

