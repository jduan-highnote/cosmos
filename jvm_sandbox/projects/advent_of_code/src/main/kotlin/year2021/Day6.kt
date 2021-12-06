package year2021

import java.io.File
import java.math.BigInteger

fun countFish(inputFile: String, days: Int): BigInteger {
    // keep track of the occurrences of each timer
    var map = mutableMapOf<Int, BigInteger>()
    File(inputFile).readLines().first().split(",").map { Integer.parseInt(it) }.forEach { timer ->
        if (map.containsKey(timer)) {
            map[timer] = map[timer]!! + BigInteger.ONE
        } else {
            map[timer] = BigInteger.ONE
        }
    }

    repeat(days) {
        map = nextDay(map.toMap())
    }

    var count = BigInteger.valueOf(0L)
    map.values.forEach { value -> count = count.add(BigInteger.valueOf(value.toLong()))}

    return count
}

internal fun nextDay(timers: Map<Int, BigInteger>): MutableMap<Int, BigInteger> {
    val newTimers = mutableMapOf<Int, BigInteger>()
    timers.forEach {
        val timer = it.key
        val count = it.value
        if (timer != 0) {
            newTimers[timer - 1] =  count
        }
    }

    if (timers.containsKey(0)) {
        if (newTimers.containsKey(6)) {
            newTimers[6] = newTimers[6]!! + timers[0]!!
        } else {
            newTimers[6] = timers[0]!!
        }
        // 0 changes into 6 and adds 8
        if (newTimers.containsKey(8)) {
            newTimers[8] = newTimers[8]!! + timers[0]!!
        } else {
            newTimers[8] = timers[0]!!
        }
    }

    return newTimers
}
