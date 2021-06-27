object Solution {
    def findLucky(arr: Array[Int]): Int = {
        import scala.collection.mutable.Map
        val rc: Map[Int, Int] = Map.empty
        for(x <- arr) {
            if(rc.contains(x)) rc(x) += 1;
            else rc += (x -> 1);
        }
        val xs = rc.filter { case (k, v) => k == v }.keys
        if(xs.isEmpty) -1 else xs.max
    }
}
