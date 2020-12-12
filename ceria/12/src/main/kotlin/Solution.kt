import java.io.File
import kotlin.math.roundToInt

val floor = '.'
val seat = 'L'
val occupied = '#'

fun main(args : Array<String>) {
    val input = File(args.first()).readLines()
    println("Solution 1: ${solution1(input)}")
    println("Solution 2: ${solution2(input)}")
}

private fun solution1(input :List<String>) :Int {
    var currPos = Pair<Int, Int>(0, 0)
    var facing = 'E'
    val directionOrder = listOf<Char>('E', 'S', 'W', 'N')
    val degreeTranslater = mapOf<Int, Int>(90 to 1, 180 to 2, 270 to 3, 360 to 4)
    for (nav in input) {
        val units = nav.substring(1).toInt()
        when (nav[0]) {
            'N' -> { currPos = Pair<Int, Int>(currPos.first, currPos.second + units) }
            'S' -> { currPos = Pair<Int, Int>(currPos.first, currPos.second - units) }
            'E' -> { currPos = Pair<Int, Int>(currPos.first + units, currPos.second) }
            'W' -> { currPos = Pair<Int, Int>(currPos.first - units, currPos.second) }
            'L' -> { 
                val currDirPtr = directionOrder.indexOf(facing)
                val dirOrderPtr = degreeTranslater.get(units)!!
                facing = directionOrder.get( Math.floorMod( currDirPtr - dirOrderPtr, directionOrder.size) )
            }
            'R' -> { 
                val currDirPtr = directionOrder.indexOf(facing)
                val dirOrderPtr = degreeTranslater.get(units)!!
                facing = directionOrder.get( Math.floorMod( currDirPtr + dirOrderPtr, directionOrder.size) )
            }
            'F' -> {
                when (facing) {
                    'N' -> { currPos = Pair<Int, Int>(currPos.first, currPos.second + units) }
                    'S' -> { currPos = Pair<Int, Int>(currPos.first, currPos.second - units) }
                    'E' -> { currPos = Pair<Int, Int>(currPos.first + units, currPos.second) }
                    'W' -> { currPos = Pair<Int, Int>(currPos.first - units, currPos.second) }
                }
             }
        }
    }

    return Math.abs(currPos.first) + Math.abs(currPos.second)
}

private fun solution2(input :List<String>) :Int {
    var currPos = Pair<Int, Int>(0, 0)
    var wayPoint = Pair<Int, Int>(10, 1)
    val degreeTranslater = mapOf<Int, Int>(90 to 270, 180 to 180, 270 to 90, 360 to 0)
    for (nav in input) {
        val units = nav.substring(1).toInt()
        when (nav[0]) {
            'N' -> { wayPoint = Pair<Int, Int>(wayPoint.first, wayPoint.second + units) }
            'S' -> { wayPoint = Pair<Int, Int>(wayPoint.first, wayPoint.second - units) }
            'E' -> { wayPoint = Pair<Int, Int>(wayPoint.first + units, wayPoint.second) }
            'W' -> { wayPoint = Pair<Int, Int>(wayPoint.first - units, wayPoint.second) }
            'L' -> { 
                var x = (wayPoint.first * Math.cos(Math.toRadians(units.toDouble()))) - (wayPoint.second * Math.sin(Math.toRadians(units.toDouble())))                
                var y = (wayPoint.second * Math.cos(Math.toRadians(units.toDouble()))) + (wayPoint.first * Math.sin(Math.toRadians(units.toDouble())))
                wayPoint = Pair<Int, Int>(x.roundToInt(), y.roundToInt())
            }
            'R' -> { 
                var x = (wayPoint.first * Math.cos(Math.toRadians(degreeTranslater.get(units)!!.toDouble()))) - (wayPoint.second * Math.sin(Math.toRadians(degreeTranslater.get(units)!!.toDouble())))               
                var y = (wayPoint.second * Math.cos(Math.toRadians(degreeTranslater.get(units)!!.toDouble()))) + (wayPoint.first * Math.sin(Math.toRadians(degreeTranslater.get(units)!!.toDouble())))
                wayPoint = Pair<Int, Int>(x.roundToInt(), y.roundToInt())
            }
            'F' -> {
                for (i in 0 until units) {
                    currPos = Pair<Int, Int>(currPos.first + wayPoint.first, currPos.second + wayPoint.second) 
                }   
             }
        }
    }

    return Math.abs(currPos.first) + Math.abs(currPos.second)
}