object Solution {
    def canArrange(arr: Array[Int], k: Int): Boolean = {
        val mod = Array.fill(k)(0)
        for(num <- arr) {
            mod((num % k + k) % k) += 1
        }
        for(i <- Range(1, k / 2 + 1)) {
            if(mod(i) != mod(k - i)) return false
        }
        mod(0) % 2 == 0
    }
}
