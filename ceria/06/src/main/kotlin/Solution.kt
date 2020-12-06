import java.io.File;

fun main(args : Array<String>) {
    val input = File(args.first()).readLines()
    println("Solution 1: ${solution1(getGroups(input))}")
    println("Solution 2: ${solution2(getGroups(input))}")
}

private fun solution1(groups :List<Pair<Int, String>>) :Int {
    return groups.map{ it.second.toSet().count() }.sum()
}
    
private fun solution2(groups :List<Pair<Int, String>>) :Int {
    return groups.map{ pairIt ->
        pairIt.second.groupingBy { it }.eachCount().filter { 
            it.value == pairIt.first
        }.size
    }.sum()
}

private fun getGroups(input: List<String>) :List<Pair<Int, String>> {
    var groups = mutableListOf<Pair<Int, String>>()
    var completeGroup = ""
    var numPeopleInGroup = 0 
    for (line in input) {
        if (line.isNullOrBlank()) {
            groups.add(Pair(numPeopleInGroup, completeGroup))
            completeGroup = ""
            numPeopleInGroup = 0
            continue
        }
        numPeopleInGroup++
        completeGroup = completeGroup.plus(line)
    }

    groups.add(Pair(numPeopleInGroup, completeGroup))
    return groups
}