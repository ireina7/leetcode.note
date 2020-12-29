object Solution {
    import scala.collection.mutable.Map
    def numDecodings(s: String): Int = {
        if(s == null || s.isEmpty) return 0
            val memo = Map[Int, Int]()

        def count(idx: Int): Int = idx match {
            case i if i == s.length => 1
            case i if s(i) == '0' => 0
            case i if i == s.length - 1 => 1
            case i if memo.contains(i) => memo(i)
            case i => {
                var ans = count(i + 1)
                ans += (if(s.substring(i, i + 2).toInt <= 26) count(i + 2) else 0)
                memo += (i -> ans)
                ans
            }
            }
            count(0)
    }
}
