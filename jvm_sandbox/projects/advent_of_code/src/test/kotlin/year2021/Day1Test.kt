package year2021

import kotlin.test.assertEquals
import org.junit.Test

class Day1Test {
  @Test fun test1() {
    val inputFile = System.getProperty("user.dir") + "/src/test/kotlin/year2021/Day1Input.txt"
    assertEquals(1616, countIncreases(inputFile))
  }

  @Test fun test2() {
    val inputFile = System.getProperty("user.dir") + "/src/test/kotlin/year2021/Day1Input.txt"
    assertEquals(1645, countIncreasesSlidingWindow(inputFile))
  }
}
