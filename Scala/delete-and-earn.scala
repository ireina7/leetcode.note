object Solution {
    
    def deleteAndEarn(nums: Array[Int]): Int = {
        val maxN = 10002
        val sums = Array.fill(maxN)(0)
        nums.foreach { n =>
            sums(n) += n
        }
        for (i <- 2 until sums.length) {
            sums(i) = sums(i - 1) max (sums(i - 2) + sums(i))
        }
        sums.last
    }
}




