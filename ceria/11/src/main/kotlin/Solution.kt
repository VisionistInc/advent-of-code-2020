import java.io.File;
import java.util.LinkedList;

val floor = '.'
val seat = 'L'
val occupied = '#'

fun main(args : Array<String>) {
    val input = File(args.first()).readLines()
    // println("Solution 1: ${solution1(input)}")
    println("Solution 2: ${solution2(input)}")
}

private fun solution1(input :List<String>) :Int {
    var openSeats = mutableListOf<Pair<Int, Int>>()
    var occupiedSeats = mutableListOf<Pair<Int, Int>>()
    var floorSpace = mutableListOf<Pair<Int, Int>>()
    val xMax = input.first().length
    var yMax = input.size

    input.forEachIndexed { y, line ->
        for (x in line.indices) {
            when (line[x]) {
                seat -> { openSeats.add(Pair(x, y)) }
                floor -> { floorSpace.add(Pair(x, y)) }
            }
        }
    }

    var stable = false
    var openToOccupied = mutableListOf<Pair<Int, Int>>()
    var occupiedToOpen = mutableListOf<Pair<Int, Int>>()

    while (!stable) {
        for (seat in openSeats) {
            val adjacentSeats = checkAdjacentSeats(seat).filter {
                it.first > -1 && it.second > -1 && it.first < xMax && it.second < yMax
            }
            
            if (openSeats.union(floorSpace).containsAll(adjacentSeats)) {
               openToOccupied.add(seat)
            } 
        } 

        for (seat in occupiedSeats) {
            val adjacentSeats = checkAdjacentSeats(seat)
            if (occupiedSeats.intersect(adjacentSeats).size >= 4 ) {
                occupiedToOpen.add(seat)
            }
        }

        if (openToOccupied.isEmpty() && occupiedToOpen.isEmpty()) {
            stable = true
        }       

        openSeats.removeAll(openToOccupied)
        occupiedSeats.addAll(openToOccupied)
        openToOccupied.clear()
        occupiedSeats.removeAll(occupiedToOpen)
        openSeats.addAll(occupiedToOpen)
        occupiedToOpen.clear()
    }

    return occupiedSeats.size
}

private fun checkAdjacentSeats(seat :Pair<Int, Int>) :List<Pair<Int, Int>> {
    return listOf(
        Pair(seat.first + 1, seat.second + 1),
        Pair(seat.first + 1, seat.second - 1),
        Pair(seat.first - 1, seat.second + 1),
        Pair(seat.first - 1, seat.second - 1),
        Pair(seat.first - 1, seat.second),
        Pair(seat.first + 1, seat.second),
        Pair(seat.first, seat.second + 1),
        Pair(seat.first, seat.second - 1 ),  
    )
} 

private fun solution2(input :List<String>) :Int {
    return 0
}
