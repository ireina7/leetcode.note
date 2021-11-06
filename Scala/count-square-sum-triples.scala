object Solution {
    def countTriples(n: Int): Int = {
        val ts = for {
            a <- 1 to n
            b <- 1 to n
            c <- 1 to n
            if a * a + b * b == c * c
        } yield ()
        ts.length
    }
}