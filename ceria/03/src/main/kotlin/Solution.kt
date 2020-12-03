import java.io.File;

val tree :Char = '#'

fun main(args : Array<String>) {
    val input = File(args.first()).readLines()
    println("Solution 1: ${solution1(input)}")
    println("Solution 2: ${solution2(input)}")
}

private fun solution1(input :List<String>) :Int {
    return checkSlope(findTrees(input, 3), input.size, 3, 1)
}
    
private fun solution2(input :List<String>) :Long {
    return checkSlope(findTrees(input, 1), input.size, 1, 1).toLong() *
    checkSlope(findTrees(input, 3), input.size, 3, 1).toLong() *
    checkSlope(findTrees(input, 5), input.size, 5, 1).toLong() *
    checkSlope(findTrees(input, 7), input.size, 7, 1).toLong() *
    checkSlope(findTrees(input, 1), input.size, 1, 2).toLong()
}    

private fun findTrees(input :List<String>, xStep :Int) :List<Pair<Int, Int>> {
    val origPatternXCount = input.first().length
    val maxXNeeded = (input.size - 1) * xStep
    val numRepeatsNeeded = Math.ceil(maxXNeeded / origPatternXCount.toDouble()).toInt()
    val trees = mutableListOf<Pair<Int, Int>>()

    input.forEachIndexed { y, line ->
        val completeLine = line.repeat(numRepeatsNeeded)
        for (x in completeLine.indices) {
            if (completeLine[x] == tree) {
                trees.add(Pair(x, y))
            }
        }
    }
    return trees
}

private fun checkSlope(trees :List<Pair<Int, Int>>, maxY :Int, xStep :Int, yStep :Int) :Int {
    var x = 0
    var y = 0
    var encounteredTreeCount = 0

    while (y < maxY) {
        if (trees.contains(Pair(x, y))) {
            encounteredTreeCount++
        }
        x += xStep
        y += yStep
    }
    return encounteredTreeCount
}
