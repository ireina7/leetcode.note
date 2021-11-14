object Solution {
    def minMutationDirty(start: String, end: String, bank: Array[String]): Int = {
        import scala.collection.mutable
        def diffCount(s1: String, s2: String) =
            s1.zip(s2).count { case (c1, c2) => c1 != c2 } == 1

        val toVisit = mutable.Queue((start, 0))
        val visited = mutable.Set(start)

        while (toVisit.nonEmpty) {
            val (curr, steps) = toVisit.dequeue()
            if (curr == end) return steps

            bank.foreach { next =>
                if (!visited.contains(next) && diffCount(curr, next)) {
                    toVisit.enqueue((next, steps + 1))
                    visited.add(next)
                }
            }
        }
        -1
    }
    
    def minMutation(start: String, end: String, bank: Array[String]): Int = {
        def diffCount(s1: String, s2: String) =
            s1.zip(s2).count { case (c1, c2) => c1 != c2 } == 1
        
        def bfs(curr: Set[String], bank: Set[String]): Option[Int] = {
            if (curr.isEmpty) return None
            if (curr.contains(end)) Some(0)
            else {
                val neighbours = 
                    bank.filter(s => curr.exists(c => diffCount(c, s)))
                for {
                    rest <- bfs(neighbours, bank diff neighbours)
                } yield 1 + rest
            }
        }
        bfs(Set(start), bank.toSet).getOrElse(-1)
    }
}