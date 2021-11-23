object Solution {
    def numRabbits(answers: Array[Int]): Int = {
        answers
            .groupBy(identity)
            .mapValues(_.length)
            .map { case (i, n) => 
                if (i >= n - 1) i + 1 else {
                    val r = n % (i + 1)
                    n - r + (if (r == 0) 0 else i + 1)
                }
            }.sum
    }
}