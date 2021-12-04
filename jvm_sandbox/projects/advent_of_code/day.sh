#!/usr/bin/env bash

day=4

cp src/main/kotlin/year2021/Day1.kt src/main/kotlin/year2021/Day${day}.kt
cp src/test/kotlin/year2021/Day1Test.kt src/test/kotlin/year2021/Day${day}Test.kt
mv ~/Downloads/input.txt src/test/kotlin/year2021/Day${day}Input.txt
