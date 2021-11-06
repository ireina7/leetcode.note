object Solution {
    def getPermutation(n: Int, k: Int): String = {
        val ps = (1 to n).permutations.toVector
        ps(k - 1).mkString
    }
}