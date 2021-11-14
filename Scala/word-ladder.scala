object Solution {
    def ladderLength(beginWord: String, endWord: String, wordList: List[String]): Int = {
        def diffCount(s1: String, s2: String) =
            s1.zip(s2).count { case (c1, c2) => c1 != c2 } == 1
        
        def bfs(curr: Set[String], bank: Set[String]): Option[Int] = {
            if (curr.isEmpty) return None
            if (curr.contains(endWord)) Some(1)
            else {
                val neighbours = 
                    bank.filter(s => curr.exists(c => diffCount(c, s)))
                for {
                    rest <- bfs(neighbours, bank diff neighbours)
                } yield 1 + rest
            }
        }
        bfs(Set(beginWord), wordList.toSet).getOrElse(0)
    }
}