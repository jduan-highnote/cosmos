package year2021

import java.io.File
import java.lang.Integer.max
import java.lang.Integer.min
import kotlin.math.abs

fun countPoints(inputFile: String): Int {
    var dict = mutableMapOf<Pair<Int, Int>, Int>()

    File(inputFile).forEachLine { line ->
        val (from, to) = line.split(" -> ")
        val (x1, y1) = from.split(",").map { Integer.parseInt(it) }
        val (x2, y2) = to.split(",").map { Integer.parseInt(it) }
        if (x1 == x2) {
            val x = x1
            for (y in min(y1, y2)..max(y1, y2)) {
                val pair = Pair(x, y)
                if (dict.containsKey(pair)) {
                    dict[pair] = dict[pair]!! + 1
                } else {
                    dict[pair] = 1
                }
            }
        }
        if (y1 == y2) {
            val y = y1
            for (x in min(x1, x2)..max(x1, x2)) {
                val pair = Pair(x, y)
                if (dict.containsKey(pair)) {
                    dict[pair] = dict[pair]!! + 1
                } else {
                    dict[pair] = 1
                }
            }
        }

        println("processed line: $line")
    }

    var count = 0
    dict.forEach { (pair, c) ->
        if (c > 1) {
            count++
        }
    }

    return count
}

fun countPoints2(inputFile: String): Int {
    var dict = mutableMapOf<Pair<Int, Int>, Int>()

    File(inputFile).forEachLine { line ->
        val (from, to) = line.split(" -> ")
        val (x1, y1) = from.split(",").map { Integer.parseInt(it) }
        val (x2, y2) = to.split(",").map { Integer.parseInt(it) }

        val points = if (x1 <= x2) {
            helper(x1, y1, x2, y2)
        } else {
            helper(x2, y2, x1, y1)
        }

        points.forEach { pair ->
            if (dict.containsKey(pair)) {
                dict[pair] = dict[pair]!! + 1
            } else {
                dict[pair] = 1
            }
        }
        println("processed line: $line")
    }

    var count = 0
    dict.forEach { (pair, c) ->
        if (c > 1) {
            count++
        }
    }

    return count
}

// Find the points between two points
fun helper(x1: Int, y1: Int, x2: Int, y2: Int): List<Pair<Int, Int>> {
    val dx = abs(x1 - x2)
    val dy = abs(y1 - y2)
    val points = mutableListOf<Pair<Int, Int>>()

    if (x1 == x2) {
        val x = x1
        for (y in min(y1, y2)..max(y1, y2)) {
            points.add(Pair(x, y))
        }
    } else {
        if (y1 < y2) {
            var x = x1
            for (y in y1..y2 step (dy / dx)) {
                points.add(Pair(x, y))
                x++
            }
        } else if (y1 == y2) {
            var x = x1
            repeat(dx + 1) {
                points.add(Pair(x, y1))
                x++
            }
        } else {
            var x = x1
            for (y in y1 downTo y2 step (dy / dx)) {
                points.add(Pair(x, y))
                x++
            }
        }
    }

    return points
}
