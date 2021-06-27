object Solution {
    def canReorderDoubled(arr: Array[Int]): Boolean = {

        def search(xs: Array[Int]): Boolean = {
            if(xs.isEmpty) return true

            val i = xs.indexOf(xs.head * 2)
            if(i == -1) false
            else search(xs.slice(1, i) ++ xs.slice(i + 1, xs.length))
        }
        val pos = arr.filter(_ >= 0).sorted
        val neg = arr.filter(_ < 0).sorted.reverse
        if(pos.length % 2 != 0 || neg.length % 2 != 0) {
            return false
        }
        search(pos) && search(neg)
    }
}
