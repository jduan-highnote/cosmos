package year2021

import kotlin.test.assertEquals
import org.junit.Test

class Day2Test {
  @Test fun test1() {
    val inputFile = System.getProperty("user.dir") + "/src/test/kotlin/year2021/Day2Input.txt"
    val pair = calculatePosition(inputFile)
    assertEquals(1636725, pair.first * pair.second)
  }

  @Test fun test2() {
    val inputFile = System.getProperty("user.dir") + "/src/test/kotlin/year2021/Day2Input.txt"
    val pair = calculatePosition2(inputFile)
    assertEquals(1872757425, pair.first * pair.second)
  }
}
