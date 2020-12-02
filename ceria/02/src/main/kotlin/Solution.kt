import java.io.File;


fun main(args : Array<String>) {
    val input = File(args.first()).readLines()
    println("Solution 1: ${solution1(input)}")
    println("Solution 2: ${solution2(input)}")
}

private fun solution1(input: List<String>) :Int {
    var validPasswordIndexes =  mutableListOf<Int>()
    input.forEachIndexed { index, line ->
        val parts = line.split(" ")
        val minmax = parts.get(0).split("-").map{ it.toInt() }
        val range = IntRange(minmax.get(0), minmax.get(1))

        val countInPasswd = parts.get(2).count{ parts.get(1).dropLast(1).contains(it) }
        if (range.contains(countInPasswd)) {
            validPasswordIndexes.add(index)
        }
    }
    return validPasswordIndexes.size
}
    
private fun solution2(input: List<String>) :Int {
    var validPasswordIndexes =  mutableListOf<Int>()
    input.forEachIndexed { index, line ->
        val parts = line.split(" ")
        val minmax = parts.get(0).split("-").map{ it.toInt() }
        val range = IntRange(minmax.get(0), minmax.get(1))

        if ((parts.get(2).get(range.start - 1) == parts.get(1).dropLast(1).single()).xor(
            (parts.get(2).get(range.endInclusive - 1) == parts.get(1).dropLast(1).single())) ) {
                validPasswordIndexes.add(index)
            } 
    }
    return validPasswordIndexes.size
}    