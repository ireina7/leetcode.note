/**
 * One naive solution
*/
object Solution {
    def findSubstring(s: String, words: Array[String]): List[Int] = {
        if (s.isEmpty || words.isEmpty) {
            return Nil
        }
        val counts  = words.groupBy(identity).mapValues(_.length).toMap
        val wordLen = words.head.length
        
        s.toSeq
            .sliding(words.length * wordLen, 1)
            .zipWithIndex
            .toList
            .flatMap { case (ss, i) =>
                val seen = ss
                    .sliding(wordLen, wordLen)
                    .map(_.unwrap)
                    .toList
                    .groupBy(identity)
                    .mapValues(_.length)
                    .toMap
                if (seen == counts) List(i) else List()
            }
    }
}