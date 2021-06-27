object Solution {

    class Lazy[T] (expr : => T) {
      lazy val value = expr
      def apply(): T = value
    }
    object Lazy{ def apply[T](expr : => T) = new Lazy(expr) }

    def minDistance(s1: String, s2: String): Int = {
        def go(i: Int, j: Int): Lazy[Int] = Lazy {
            if (i == 0) j
            else if (j == 0) i
            else if (s1(i - 1) == s2(j - 1)) mem(i - 1)(j - 1)()
            else (mem(i - 1)(j)() + 1) min
                 (mem(i)(j - 1)() + 1) min (mem(i - 1)(j - 1)() + 1)
        }
        lazy val mem = Array.tabulate[Lazy[Int]](s1.length + 1, s2.length + 1)(go)
        go(s1.length, s2.length)()
    }
}
