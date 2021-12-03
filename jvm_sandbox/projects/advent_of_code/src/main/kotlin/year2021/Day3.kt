package year2021

import java.io.File
import java.lang.RuntimeException
import java.lang.StringBuilder

fun calculatePowerConsumption(inputFile: String): Int {
    var gammaRate = StringBuilder()
    var epsilonRate = StringBuilder()

    val numbers = File(inputFile).readLines().map { line ->
        line.toCharArray().map { Integer.parseInt(it.toString()) }
    }

    for (i in 0 until numbers[0].size) {
        var zeros = 0
        var ones = 0
        numbers.forEach { number ->
            if (number[i] == 0) {
                zeros++
            } else {
                ones++
            }
        }

        if (zeros > ones) {
            gammaRate.append("0")
            epsilonRate.append("1")
        } else {
            gammaRate.append("1")
            epsilonRate.append("0")
        }
    }

    return Integer.parseInt(gammaRate.toString(), 2) * Integer.parseInt(epsilonRate.toString(), 2)
}

fun calculateLifeSupport(inputFile: String): Int {
    return Integer.parseInt(calculateOxygenRating(inputFile).joinToString(separator = ""), 2) *
            Integer.parseInt(calculateCO2Rating(inputFile).joinToString(separator = ""), 2)
}

fun calculateOxygenRating(inputFile: String): List<Int> {
    var gammaRate = StringBuilder()
    var epsilonRate = StringBuilder()

    var numbers = File(inputFile).readLines().map { line ->
        line.toCharArray().map { Integer.parseInt(it.toString()) }
    }

    for (i in 0 until numbers[0].size) {
        var zeros = mutableListOf<List<Int>>()
        var ones = mutableListOf<List<Int>>()
        numbers.forEach { number ->
            if (number[i] == 0) {
                zeros.add(number)
            } else {
                ones.add(number)
            }
        }

        if (ones.size >= zeros.size) {
            numbers = ones
        } else {
            numbers = zeros
        }

        if (numbers.size == 1) {
            return numbers[0]
        }
    }

    throw RuntimeException("Couldn't be found")
}

fun calculateCO2Rating(inputFile: String): List<Int> {
    var gammaRate = StringBuilder()
    var epsilonRate = StringBuilder()

    var numbers = File(inputFile).readLines().map { line ->
        line.toCharArray().map { Integer.parseInt(it.toString()) }
    }

    for (i in 0 until numbers[0].size) {
        var zeros = mutableListOf<List<Int>>()
        var ones = mutableListOf<List<Int>>()
        numbers.forEach { number ->
            if (number[i] == 0) {
                zeros.add(number)
            } else {
                ones.add(number)
            }
        }

        if (zeros.size <= ones.size) {
            numbers = zeros
        } else {
            numbers = ones
        }

        if (numbers.size == 1) {
            return numbers[0]
        }
    }

    throw RuntimeException("Couldn't be found")
}
