package year2021

import java.io.File
import kotlin.math.abs

fun calculateFuel(inputFile: String): Int {
    val positions = File(inputFile).readLines().first().split(",").map { Integer.parseInt(it) }

    var minFuel: Int? = null
    positions.forEach { base ->
        var fuel = 0
        positions.forEach { pos ->
            fuel += abs(pos - base)
        }

        if (minFuel == null) {
            minFuel = fuel
        } else {
            if (fuel < minFuel!!) {
                minFuel = fuel
            }
        }
    }

    return minFuel!!
}

fun calculateFuel2(inputFile: String): Int {
    val positions = File(inputFile).readLines().first().split(",").map { Integer.parseInt(it) }

    var minFuel: Int? = null
    positions.forEach { base ->
        var fuel = 0
        positions.forEach { pos ->
            val distance = abs(pos - base)
            fuel += (1 + distance) * distance / 2
        }

        if (minFuel == null) {
            minFuel = fuel
        } else {
            if (fuel < minFuel!!) {
                minFuel = fuel
            }
        }
    }

    return minFuel!!
}

