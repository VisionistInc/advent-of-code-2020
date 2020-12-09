import java.io.File;

fun main(args : Array<String>) {
    val input = File(args.first()).readLines()
    println("Solution 1: ${solution1(input)}")
    println("Solution 2: ${solution2(input)}")
}

private fun solution1(input :List<String>) :Long {
    val preamble = 25

    for (i in input.indices) {
        if (i < preamble) {
            continue
        }

        val availableNums = input.subList(i - preamble, i).map{ it.toLong() }
        val sumNum = input.get(i).toLong()
        var valid = false
        for (avail1 in availableNums.indices) {
            if (avail1 < availableNums.size - 1) {
                for (avail2 in availableNums.indices) {
                    if (avail1 != avail2) {
                        if (availableNums.get(avail1) + availableNums.get(avail2) == sumNum) {
                            valid = true
                            break
                        }
                    }
                }

                if (valid) {
                    break
                }
             }
        }

        if (!valid) {
            return sumNum
        }
    }

    return 0
}

private fun solution2(input :List<String>) :Long {
    val invalidNum = solution1(input)
    for (i in input.indices) {
        val subInput = input.drop(i).map{ it.toLong() }
        var subInputIndex = -1
        var runningSum = 0L
        for (contigIndex in subInput.indices) {
            runningSum += subInput.get(contigIndex)
            if (runningSum > invalidNum ) {
                break
            }

            if (runningSum == invalidNum) {
                subInputIndex = contigIndex
                break
            }
        }

        if (subInputIndex != -1) {
            val contigValues = subInput.subList(0, subInputIndex + 1)
            return contigValues.minOrNull()!! + contigValues.maxOrNull()!!
        }
    }
     return 0
}
