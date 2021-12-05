package year2021

import kotlin.test.assertEquals
import org.junit.Test

class Day5Test {
  @Test fun test1() {
    val inputFile = System.getProperty("user.dir") + "/src/test/kotlin/year2021/Day5Input.txt"
    assertEquals(6856, countPoints(inputFile))
  }

  @Test fun test2() {
    val inputFile = System.getProperty("user.dir") + "/src/test/kotlin/year2021/Day5Input.txt"
    assertEquals(20666, countPoints2(inputFile))
  }

}
