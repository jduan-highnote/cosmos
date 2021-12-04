package year2021

import java.io.File
import java.lang.RuntimeException

class Board(private val board: List<List<Int>>) {
    private val state: MutableMap<Pair<Int, Int>, Boolean> = mutableMapOf()

    init {
        for (i in board.indices) {
            for (j in 0 until board[0].size) {
                state[Pair(i, j)] = false
            }
        }
    }

    fun playBingo(num: Int) {
        for (i in board.indices) {
            for (j in 0 until board[0].size) {
                if (board[i][j] == num) {
                    state[Pair(i, j)] = true
                }
            }
        }
    }

    fun checkState(): Boolean {
        // check all rows
        for (i in board.indices) {
            val row = mutableListOf<Boolean>()
            for (j in 0 until board[0].size) {
                row.add(state[Pair(i, j)]!!)
            }
            if (row.all { it }) {
                return true
            }
        }

        // check all columns
        for (col in 0 until board[0].size) {
            val bools = mutableListOf<Boolean>()
            for (row in board.indices) {
                bools.add(state[Pair(row, col)]!!)
            }
            if (bools.all { it }) {
                return true
            }
        }

        return false
    }

    fun remainingSum(): Int {
        var sum = 0
        for (i in board.indices) {
            for (j in 0 until board[0].size) {
                if (! state[Pair(i, j)]!!) {
                    sum += board[i][j]
                }
            }
        }

        return sum
    }
}

fun countScore(inputFile: String): Int {
    val (firstLine, boards) = processInput(inputFile)
    firstLine.forEach { num ->
        boards.forEach { board ->
            board.playBingo(num)
            if (board.checkState()) {
                return num * board.remainingSum()
            }
        }
    }

    throw RuntimeException("Failed to find a board that wins")
}

// let the squid win
fun countScore2(inputFile: String): Int {
    var (firstLine, boards) = processInput(inputFile)
    var count = 0
    val totalBoards = boards.size
    firstLine.forEach { num ->
        val remainingBoards = mutableListOf<Board>()
        boards.forEach { board ->
            board.playBingo(num)
            if (board.checkState()) {
                count++
            } else {
                remainingBoards.add(board)
            }
            if (count == totalBoards) {
                // last board
                return num * board.remainingSum()
            }
        }

        boards = remainingBoards
    }

    throw RuntimeException("Failed to find a board that wins last")
}

private fun processInput(inputFile: String): Pair<List<Int>, List<Board>> {
    val file = File(inputFile)
    val lines = file.readLines()
    val firstLine = lines.first().split(",").map { Integer.parseInt(it) }
    val boards = mutableListOf<Board>()

    val board = mutableListOf<List<Int>>()
    lines.drop(1).forEach { line ->
        if (line.isEmpty()) {
            if (board.isNotEmpty()) {
                boards.add(Board(board.toList()))
                board.clear()
            }
        } else {
            board.add(line.split(" ").filterNot { it.isEmpty() }.map { Integer.parseInt(it) })
        }
    }

    return Pair(firstLine, boards)
}