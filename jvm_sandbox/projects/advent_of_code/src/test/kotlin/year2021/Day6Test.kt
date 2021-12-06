package year2021

import kotlin.test.assertEquals
import org.junit.Test
import java.math.BigInteger

class Day6Test {
  @Test fun test1() {
    val inputFile = System.getProperty("user.dir") + "/src/test/kotlin/year2021/Day6Input.txt"
    assertEquals(BigInteger.valueOf(362666L), countFish(inputFile, 80))
  }

  @Test fun test2() {
    val inputFile = System.getProperty("user.dir") + "/src/test/kotlin/year2021/Day6Input.txt"
    println(countFish(inputFile, 256).toString())
    assertEquals(BigInteger.valueOf(1640526601595), countFish(inputFile, 256))
  }

  @Test fun testNextDay() {
    val timers = mapOf(
      3 to BigInteger.valueOf(2L),
      4 to BigInteger.valueOf(1L),
      1 to BigInteger.valueOf(1L),
      2 to BigInteger.valueOf(1L))
    val expected = mapOf(
      2 to BigInteger.valueOf(2L),
      3 to BigInteger.valueOf(1L),
      0 to BigInteger.valueOf(1L),
      1 to BigInteger.valueOf(1L))
    assertEquals(expected, nextDay(timers))
  }

  @Test fun testNextDay2() {
    val timers = mapOf(
      2 to BigInteger.valueOf(3L),
      3 to BigInteger.valueOf(2L),
      0 to BigInteger.valueOf(1L),
      1 to BigInteger.valueOf(1L),
      4 to BigInteger.valueOf(2L),
      5 to BigInteger.valueOf(1L))
    val expected = mapOf(
      0 to BigInteger.valueOf(1L),
      1 to BigInteger.valueOf(3L),
      2 to BigInteger.valueOf(2L),
      3 to BigInteger.valueOf(2L),
      4 to BigInteger.valueOf(1L),
      6 to BigInteger.valueOf(1L),
      8 to BigInteger.valueOf(1L))
    assertEquals(expected, nextDay(timers))
  }
}
