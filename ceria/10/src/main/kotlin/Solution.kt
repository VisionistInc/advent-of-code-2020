import java.io.File;
import java.util.Collections;

fun main(args : Array<String>) {
    val input = File(args.first()).readLines().map{ it.toInt() }
    println("Solution 1: ${solution1(input)}")
    println("Solution 2: ${solution2(input)}")
}

private fun solution1(input :List<Int>) :Int {
    var highestAdapter = input.maxOrNull()!!
    var joltageDiffs = mutableMapOf<Int, MutableList<Int>>()
    joltageDiffs.put(1, mutableListOf<Int>())
    joltageDiffs.put(2, mutableListOf<Int>())
    joltageDiffs.put(3, mutableListOf<Int>())

    var currentJoltage = 0

    while (currentJoltage != highestAdapter) {
        for (i in 1..3) {
            val nextAdapter = currentJoltage + i
            if (input.contains(nextAdapter)) {
                var adapters = joltageDiffs.get(i)!!
                adapters.add(nextAdapter)
                joltageDiffs.put(i, adapters)
                currentJoltage += i
                break
            }
        }
    }
    var adapters = joltageDiffs.get(3)!!
    adapters.add( highestAdapter + 3)
    joltageDiffs.put(3, adapters)
    return joltageDiffs.get(1)!!.size * joltageDiffs.get(3)!!.size
}

private fun solution2(input :List<Int>) :Long {
    val startVals = input.toMutableList<Int>()
    startVals.add(input.maxOrNull()!! + 3)
    val sorted = startVals.toIntArray()
    sorted.sort()

    var full = calculate(sorted)
    if (sorted[0] < 3){
        full += calculate(sorted.sliceArray(1..sorted.size-1))
    }
    if (sorted[1] < 3){
        full += calculate(sorted.sliceArray(2..sorted.size-1))
    }

    return full
}

private fun calculate(sorted :IntArray) :Long{
    var nodes = mutableMapOf<Int, Long>()
    var replacementNodes = mutableListOf<Int>()

    nodes.put(sorted[0], 1L)

    for (sortedIndex in sorted.indices) {
        var numBranches = 1

        if (sortedIndex + 1 < sorted.size) {
            var diff = sorted[sortedIndex + 1] - sorted[sortedIndex] 
            if (diff == 3) {
                // no choices
                numBranches = 1
            }

            if (diff == 1) {
                // check next 2
                if (sortedIndex + 2 < sorted.size) {
                    if (sorted[sortedIndex + 2] - sorted[sortedIndex] == 2) {
                        numBranches = 2
                    }
                }

                if (sortedIndex + 3 < sorted.size) {
                    if (sorted[sortedIndex + 3] - sorted[sortedIndex] == 3) {
                        numBranches = 3
                    }
                }
            }

            if (diff == 2) {
                // check next 1
                if (sortedIndex + 2 < sorted.size) {
                    if (sorted[sortedIndex + 2] - sorted[sortedIndex] == 2) {
                        numBranches = 2
                    }
                }
            }
        }

        for (n in 1..numBranches) {
            if (sortedIndex + n < sorted.size) {
                replacementNodes.add(sorted[sortedIndex + n])
            }
        }

        if (sorted[sortedIndex] != sorted.last() ) {
            var numTimesToAddReplacementNodes = 0L
            if (nodes.containsKey(sorted[sortedIndex])) {
                numTimesToAddReplacementNodes = nodes.get(sorted[sortedIndex])!!
            }
            nodes.remove(sorted[sortedIndex])
            for (n in replacementNodes) {
                var count = if (nodes.containsKey(n)) nodes.get(n)!! else 0L
                count += numTimesToAddReplacementNodes
                nodes.put(n, count)
            }
            replacementNodes.clear()
        }
    }
    
     return nodes.values.sum()
}
