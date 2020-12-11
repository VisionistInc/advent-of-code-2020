import java.io.File;
import java.util.LinkedList;

val floor = '.'
val seat = 'L'
val occupied = '#'

fun main(args : Array<String>) {
    val input = File(args.first()).readLines()
    println("Solution 1: ${solution1(input)}")
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

    var openToOccupied = mutableListOf<Pair<Int, Int>>()
    var occupiedToOpen = mutableListOf<Pair<Int, Int>>()

    while (true) {
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
            break
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

private fun solution2(input :List<String>) :Int {
    var openSeats = mutableListOf<Pair<Int, Int>>()
    var occupiedSeats = mutableListOf<Pair<Int, Int>>()
    var floorSpace = mutableListOf<Pair<Int, Int>>()
    val xMax = input.first().length
    val yMax = input.size

    input.forEachIndexed { y, line ->
        for (x in line.indices) {
            when (line[x]) {
                seat -> { openSeats.add(Pair(x, y)) }
                floor -> { floorSpace.add(Pair(x, y)) }
            }
        }
    }

    var openToOccupied = mutableListOf<Pair<Int, Int>>()
    var occupiedToOpen = mutableListOf<Pair<Int, Int>>()

        // println()
        // debug(floorSpace, openSeats, occupiedSeats, xMax, yMax)
        // println()


    while (true) {
        for (seat in openSeats) {
            val visibleSeats = checkSeatsCanSee(seat, floorSpace, xMax, yMax).filter {
                it.first > -1 && it.second > -1 && it.first < xMax && it.second < yMax
            }
            if (openSeats.containsAll(visibleSeats)) {
               openToOccupied.add(seat)
            } 
        } 

        for (seat in occupiedSeats) {
            val visibleSeats = checkSeatsCanSee(seat, floorSpace, xMax, yMax).toMutableList()
            if (occupiedSeats.intersect( visibleSeats).size >= 5 ) {
                occupiedToOpen.add(seat)
            }
        }

        if (openToOccupied.isEmpty() && occupiedToOpen.isEmpty()) {
            break
        }       

        openSeats.removeAll(openToOccupied)
        occupiedSeats.addAll(openToOccupied)
        openToOccupied.clear()
        occupiedSeats.removeAll(occupiedToOpen)
        openSeats.addAll(occupiedToOpen)
        occupiedToOpen.clear()

        // println()
        // debug(floorSpace, openSeats, occupiedSeats, xMax, yMax)
        // println()

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

private fun checkSeatsCanSee(seat :Pair<Int, Int>, floorSpace :List<Pair<Int, Int>>, xMax :Int, yMax :Int) :List<Pair<Int, Int>> {
    var canSee = mutableListOf<Pair<Int, Int>>()

    var brDiagFound = false
    var trDiagFound = false
    var rightFound = false
    var blDiagFound = false
    var tlDiagFound = false
    var leftFound = false

    for (x in 1 until xMax) {
        if (Pair(seat.first + x, seat.second + x) !in floorSpace && !brDiagFound) {
            canSee.add(Pair(seat.first + x, seat.second + x))
            brDiagFound = true
        }

        if (Pair(seat.first + x, seat.second - x) !in floorSpace && !trDiagFound) {
            canSee.add(Pair(seat.first + x, seat.second - x))
            trDiagFound = true
        }

        if (Pair(seat.first + x, seat.second) !in floorSpace && !rightFound) {
            canSee.add(Pair(seat.first + x, seat.second))
            rightFound = true
        }

        if (Pair(seat.first - x, seat.second + x) !in floorSpace && !blDiagFound) {
            canSee.add(Pair(seat.first - x, seat.second + x))
            blDiagFound = true
        }

        if (Pair(seat.first - x, seat.second - x) !in floorSpace && !tlDiagFound) {
            canSee.add(Pair(seat.first - x, seat.second - x))
            tlDiagFound = true
        }

        if (Pair(seat.first - x, seat.second) !in floorSpace && !leftFound) {
            canSee.add(Pair(seat.first - x, seat.second))
            leftFound = true
        }

        if (brDiagFound && trDiagFound && rightFound && blDiagFound && tlDiagFound && leftFound) { break }
    }

    var bottomFound = false
    var topFound = false

    for (y in 1 until yMax) {
        if (Pair(seat.first, seat.second + y) !in floorSpace && !bottomFound) {
            canSee.add(Pair(seat.first, seat.second + y))
            bottomFound = true
        }

        if (Pair(seat.first, seat.second - y) !in floorSpace && !topFound) {
            canSee.add(Pair(seat.first, seat.second - y))
            topFound = true
        }

        if (bottomFound && topFound) { break }
    }

    return canSee
}

private fun debug(floorSpace :List<Pair<Int, Int>>, openSeats :List<Pair<Int, Int>>, occupiedSeats :List<Pair<Int, Int>>, xMax :Int, yMax :Int) {
    for ( y in 0 until yMax ) {
        for (x in 0 until xMax ) {
            if (floorSpace.contains(Pair(x,y))) {
                print('.')
            } else if (openSeats.contains(Pair(x, y))) {
                print('L')
            } else if (occupiedSeats.contains(Pair(x, y))) {
                print('#')
            }
        }
        println()
    }
}