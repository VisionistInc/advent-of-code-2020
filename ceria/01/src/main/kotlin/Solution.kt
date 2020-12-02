import java.io.File;


fun main(args : Array<String>) {
    val input = File(args.first()).readLines().map { it.toInt() }
    println("Solution 1: ${solution1(input)}")
    println("Solution 2: ${solution2(input)}")
}

private fun solution1(input: List<Int>) :Int {
    for (e1 in input) {
        for (e2 in input) {
            if (e1 + e2 == 2020) {
                return e1 * e2
            }
        }
    }
    return 0
}
    
private fun solution2(input: List<Int>) :Int {
    for (e1 in input) {
        for (e2 in input) {
            for (e3 in input) {
                if (e1 + e2 + e3 == 2020) {
                    return e1 * e2 * e3
                }
            }
        }
    }
    return 0
}    