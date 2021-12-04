package year2021

import kotlin.test.assertEquals
import org.junit.Test

class Day4Test {
  @Test fun test1() {
    val inputFile = System.getProperty("user.dir") + "/src/test/kotlin/year2021/Day4Input.txt"
    assertEquals(72770, countScore(inputFile))
  }

  @Test fun test2() {
    val inputFile = System.getProperty("user.dir") + "/src/test/kotlin/year2021/Day4Input.txt"
    assertEquals(13912, countScore2(inputFile))
  }
}
