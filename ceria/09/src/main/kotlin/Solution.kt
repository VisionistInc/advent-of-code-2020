import java.io.File;

fun main(args : Array<String>) {
    val input = File(args.first()).readLines()
    println("Solution 1: ${solution1(input)}")
    println("Solution 2: ${solution2(input)}")
}

private fun solution1(input :List<String>) :Int {
    val preamble = 25

    for (i in input.indices) {
        if (i < preamble) {
            continue
        }

        val availableNums = input.subList(i - preamble, i)
        val sumNum = input.get(i).toInt()
        var valid = false
        for (avail1 in availableNums.indices) {
            if (avail1 < availableNums.size - 1) {
                for (avail2 in availableNums.indices) {
                    if (avail1 != avail2) {
                        if (availableNums.get(avail1).toInt() + availableNums.get(avail2).toInt() == sumNum) {
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

private fun solution2(input :List<String>) :Int {
     return 0
}
