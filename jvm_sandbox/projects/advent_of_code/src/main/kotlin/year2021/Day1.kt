package year2021

import java.io.File

fun countIncreases(inputFile: String): Int {
    var count = 0
    var previousLine: String? = null
    File(inputFile).forEachLine { line ->
        if (previousLine != null) {
            if (Integer.parseInt(line) > Integer.parseInt(previousLine)) {
                count += 1
            }
        }
        previousLine = line
    }
    return count
}

fun countIncreasesSlidingWindow(inputFile: String): Int {
    val lines = File(inputFile).readLines()
    var count = 0
    var first = Integer.parseInt(lines[0])
    var second = Integer.parseInt(lines[1])
    var third = Integer.parseInt(lines[2])
    var previous = first + second + third

    lines.subList(3, lines.size).forEach { line ->
        val number = Integer.parseInt(line)
        val current = number + third + second
        if (current > previous) {
            count += 1
        }
        previous = current
        second = third
        third = number
    }

    return count
}
