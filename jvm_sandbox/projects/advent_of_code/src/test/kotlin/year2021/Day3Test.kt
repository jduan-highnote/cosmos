package year2021

import kotlin.test.assertEquals
import org.junit.Test

class Day3Test {
  @Test fun test1() {
    val inputFile = System.getProperty("user.dir") + "/src/test/kotlin/year2021/Day3Input.txt"
    val pc = calculatePowerConsumption(inputFile)
    assertEquals(775304, pc)
  }

  @Test fun test2() {
    val inputFile = System.getProperty("user.dir") + "/src/test/kotlin/year2021/Day3Input.txt"
    val pc = calculateLifeSupport(inputFile)
    assertEquals(1370737, pc)
  }
}
