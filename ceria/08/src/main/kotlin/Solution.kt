import java.io.File;

fun main(args : Array<String>) {
    val input = File(args.first()).readLines()
    println("Solution 1: ${solution1(input)}")
    println("Solution 2: ${solution2(input)}")
}

private fun solution1(input :List<String>) :Int {
    var currIndex = 0
    var seenIndexes = mutableSetOf<Int>()
    var accumulator = 0

    while ( !seenIndexes.contains(currIndex) ) {
        val instruction = input.get(currIndex).split(" ")
        seenIndexes.add(currIndex)
        when (instruction.get(0)) {
            "acc" -> {
                accumulator = accumulator + Integer.parseInt(instruction.get(1))
                currIndex++
            }
            "jmp" -> {
                currIndex = currIndex + Integer.parseInt(instruction.get(1))
            }
            "nop" -> {
                currIndex++
            }
        }
    }

    return accumulator
}

private fun solution2(input :List<String>) :Int {

   for ((i, inst) in input.withIndex()) {
        if (inst.split(" ").get(0).equals("acc"))  {
            continue 
        }

        var seenIndexes = mutableSetOf<Int>()
        var currIndex = 0
        var accumulator = 0

        while ( !seenIndexes.contains(currIndex) && currIndex < input.size) {
            var instruction = input.get(currIndex).split(" ").toMutableList()
            if (currIndex == i) {
                if (instruction.get(0).equals("jmp")) {
                    instruction.set(0, "nop")
                } else {
                    instruction.set(0, "jmp")
                }
            }

            seenIndexes.add(currIndex)
            when (instruction.get(0)) {
                "acc" -> {
                    accumulator = accumulator + Integer.parseInt(instruction.get(1))
                    currIndex++
                }
                "jmp" -> {
                    currIndex = currIndex + Integer.parseInt(instruction.get(1))
                }
                "nop" -> {
                    currIndex++
                }
            }
        }

        if (currIndex == input.size) {
            return accumulator
        }
    }

    return -1
}