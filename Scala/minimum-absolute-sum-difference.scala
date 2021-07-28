object Solution {
    import Math.abs
    /**
      * O(nlog(n)) solution
    */
    def minAbsoluteSumDiff(xs: Array[Int], ys: Array[Int]): Int = {
        
        val ss = xs.sorted
        var d: Int = 0
        var sum: Long = 0
        (xs zip ys) foreach { case (x, y) =>
            val c = getClosest(y, ss)
            val a = abs(x - y)
            val b = abs(c - y)
            d = d.max(abs(a - b))
            sum += a
        }
        ((sum - d) % (Math.pow(10, 9) + 7)).toInt // This is evil! bad problem!
    }
    
    def getClosest(num: Int, xs: Seq[Int]): Int = {
        var i = 0
        var j = xs.length - 1
        
        while (i < j) {
            val (a, b) = (xs(i), xs(j))
            if (a >= num) return a
            if (b <= num) return b
            if (j - i == 1) 
                return if (abs(num - a) < abs(num - b)) a else b
            
            val k = i + (j - i) / 2
            val c = xs(k)
            if (c == num) return c
            if (c > num) j = k else i = k
        }
        xs(i)
    }
}


