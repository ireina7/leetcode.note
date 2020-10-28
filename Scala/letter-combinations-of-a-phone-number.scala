object Solution {
    /**
      * Mappings
      */
    val map: Map[Char, String] = Map(
        '2' -> "abc",
        '3' -> "def",
        '4' -> "ghi",
        '5' -> "jkl",
        '6' -> "mno",
        '7' -> "pqrs",
        '8' -> "tuv",
        '9' -> "wxyz",
    )
    /**
      * Naive recursion version, using monads...
      *
      * @param digits
      * @return
      */
    def letterCombinations(digits: String): List[String] = {
        if(digits.isEmpty) return Nil
        def getCombinations(digits: String): List[String] = {
            if(digits.isEmpty) return List("")
            for {
                c <- map(digits.head).toList
                s <- getCombinations(digits.tail)
            } yield c + s
        }
        getCombinations(digits)
    }

    /**
      * Tail recursion version!
      */
    def letterCombinations(digits: String): List[String] = {
        def rec(digits: String, acc: List[String]): List[String] = {
            if (digits == "") acc
            else rec(digits.tail, acc.flatMap(x => map(digits.head).map(y => x + y)))
        }
        if (digits == "") List() else rec(digits, List(""))
    }
}