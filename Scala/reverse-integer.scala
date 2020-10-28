object Solution {
    /**
      * The problem assume that is the returning number is overflow, then return 0,
      * which means we cannot simply use string.reverse.
      * However, we can catch this error and convert it to a normal value:)
      *
      * @param x
      * @return
      */
    def reverse(x: Int): Int = 
        try {
            val sign = if(x < 0) -1 else 1
            sign * (sign * x).toString.reverse.toInt
        } catch {
            case e: Exception => 0
        }
}