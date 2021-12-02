package year2021

import java.io.File

fun calculatePosition(inputFile: String): Pair<Int, Int> {
    var x = 0
    var y = 0
    File(inputFile).forEachLine { line ->
        val parts = line.split(' ')
        val direction = parts[0]
        val amount = Integer.parseInt(parts[1])

        when (direction) {
            "forward" -> x += amount
            "down" -> y += amount
            "up" -> y -= amount
            else -> throw Exception("Unknown direction: $direction")
        }
    }

    return Pair(x, y)
}

fun calculatePosition2(inputFile: String): Pair<Int, Int> {
    var horizontal = 0
    var aim = 0
    var depth = 0
    File(inputFile).forEachLine { line ->
        val parts = line.split(' ')
        val direction = parts[0]
        val amount = Integer.parseInt(parts[1])

        when (direction) {
            "forward" -> {
                horizontal += amount
                depth += aim * amount
            }
            "down" -> aim += amount
            "up" -> aim -= amount
            else -> throw Exception("Unknown direction: $direction")
        }
    }

    return Pair(horizontal, depth)
}
