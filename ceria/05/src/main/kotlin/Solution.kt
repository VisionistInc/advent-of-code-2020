import java.io.File;

fun main(args : Array<String>) {
    val input = File(args.first()).readLines()
    println("Solution 1: ${solution1(input)}")
    println("Solution 2: ${solution2(input)}")
}

private fun solution1(input :List<String>) :Int {
    var highestSeatID = 0
    for (boardingPass in input) {
       val seatID = getSeatID(boardingPass)
       if (seatID > highestSeatID) {
            highestSeatID = seatID
        }
    }

    return highestSeatID
}
    
private fun solution2(input :List<String>) :Int {
    var seatIDs = mutableListOf<Int>()
    for (boardingPass in input) {
        seatIDs.add(getSeatID(boardingPass))
    }
    var sortedSeatIDs = seatIDs.toSet().toIntArray()
    sortedSeatIDs.sort()

    var previousSeat = sortedSeatIDs.first()
    for (seatID in sortedSeatIDs.drop(1)) { 
        if (seatID - previousSeat == 2) {
             return seatID - 1            
        }
        previousSeat = seatID
     }
    
   return -1
}    

private fun getSeatID(boardingPass :String) :Int {
    var seatRow = IntRange(0, 127).toList()
    var seatCol = IntRange(0, 7).toList()

    boardingPass.forEach { 
        when (it) {
            'F' -> {
                val middleIndex = if (seatRow.size / 2 % 2 == 1) seatRow.size / 2 - 1 else seatRow.size / 2
                seatRow = IntRange(seatRow.first(), seatRow.get(middleIndex)).toList()
            }
            'B' -> {
                val middleIndex = seatRow.size / 2
                seatRow = IntRange(seatRow.get(middleIndex), seatRow.last()).toList()
            }
            'L' -> {
                val middleIndex = if (seatCol.size % 2 == 1) seatCol.size / 2 + 1 else seatCol.size / 2 - 1
                seatCol = IntRange(seatCol.first(), seatCol.get(middleIndex)).toList()

            }
            'R' -> {
                val middleIndex = if (seatCol.size % 2 == 1) seatCol.size / 2 + 1 else seatCol.size / 2
                seatCol = IntRange(seatCol.get(middleIndex), seatCol.last()).toList()
            }
        }
    }
    return seatRow.get(0) * 8 + seatCol.get(0)
}