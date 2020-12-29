/**
  Most simple solution
  */
object Solution {
    def strStr(haystack: String, needle: String): Int = {
        if (needle == "") 0 else haystack
            .sliding(needle.length)
            .zipWithIndex
            .collectFirst({ case (`needle`, y) => y})
            .getOrElse(-1)
    }
}
