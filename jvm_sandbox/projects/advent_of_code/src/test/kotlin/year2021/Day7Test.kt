package year2021

import kotlin.test.assertEquals
import org.junit.Test

class Day7Test {
  @Test fun test1() {
    val inputFile = System.getProperty("user.dir") + "/src/test/kotlin/year2021/Day7Input.txt"
    assertEquals(342534, calculateFuel(inputFile))
  }

  @Test fun test2() {
    val inputFile = System.getProperty("user.dir") + "/src/test/kotlin/year2021/Day7Input.txt"
    assertEquals(94004208, calculateFuel2(inputFile))
  }

}
