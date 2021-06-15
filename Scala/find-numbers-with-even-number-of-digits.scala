object Solution {
    def findNumbers(nums: Array[Int]): Int = {
        nums.map(_.toString.length).filter(_ % 2 == 0).length
    }
}
