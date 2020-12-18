import java.io.File
import java.io.Serializable

data class Quad<out X, out Y, out Z, out W>(
        val first: X,
        val second: Y,
        val third: Z,
        val fourth: W
) : Serializable {

    override fun toString(): String = "($first, $second, $third, $fourth)"

    fun <T> Quad<T, T, T, T>.toList(): List<T> = listOf(first, second, third, fourth)
}


val active = '#'

fun main(args : Array<String>) {
    val input = File(args.first()).readLines()
    println("Solution 1: ${solution1(input)}")
    println("Solution 2: ${solution2(input)}")
}

private fun solution1(input :List<String>) :Int {
  var activeCubes = mutableListOf<Triple<Int, Int, Int>>()
  var inactiveCubes = mutableListOf<Triple<Int, Int, Int>>()

  var z = 0
  input.forEachIndexed { y, line ->
    for (x in line.indices) {
        if (line[x] == active) {
          activeCubes.add(Triple(x, y, z))
        } else {
          inactiveCubes.add(Triple(x, y, z))
        }
    }
  }

  var maxX = input.first().length + 1
  var minX = 0
  var maxY = input.size + 1
  var minY = 0
  var cycles = 1

  while (cycles <= 6) {
    var addToInactiveCubes = mutableListOf<Triple<Int, Int, Int>>()
    var addToActiveCubes = mutableListOf<Triple<Int, Int, Int>>()
    minX--
    minY--

    while (z < cycles + 1 ) {
      var y = minY
      while (y < maxY) {
        var x = minX
        while (x < maxX) {
          var cube = Triple(x, y, z)
          val activeNeighborCount = activeCubes.intersect(getNeighbors(cube) ).size

          if (activeCubes.contains(cube)) {
            if (activeNeighborCount != 2 && activeNeighborCount != 3) {
              // cube becomes inactive for current z cube
              addToInactiveCubes.add(cube)
              if (z > 0) {
                addToInactiveCubes.add(Triple(cube.first, cube.second, -z))
              }
            }  
          } else {
            if (activeNeighborCount == 3) {
              addToActiveCubes.add(cube)
              if (z > 0) {
                addToActiveCubes.add(Triple(cube.first, cube.second, -z))
              }
              if (y == maxY - 1 || x == maxX - 1) {
                // increase the y on the fly to make sure we process all of the potential active cubes since the grid is infinite
                maxY++
                maxX++
              }
            }
          }
          x++
        }
        y++
      }
      z++
    }

    activeCubes.removeAll(addToInactiveCubes)
    inactiveCubes.addAll(addToInactiveCubes)
    inactiveCubes.removeAll(addToActiveCubes)
    activeCubes.addAll(addToActiveCubes)
    z = 0
    cycles++
  }

  // debug(maxX, maxY, cycles + 1, activeCubes)
  return activeCubes.size
}

private fun solution2(input :List<String>) :Int {
  var activeCubes = mutableListOf<Quad<Int, Int, Int, Int>>()
  var inactiveCubes = mutableListOf<Quad<Int, Int, Int, Int>>()

  var z = 0
  var w = 0
  input.forEachIndexed { y, line ->
    for (x in line.indices) {
        if (line[x] == active) {
          activeCubes.add(Quad(x, y, z, w))
        } else {
          inactiveCubes.add(Quad(x, y, z, w))
        }
    }
  }

  var maxX = input.first().length + 1
  var minX = 0
  var maxY = input.size + 1
  var minY = 0
  var cycles = 1

  while (cycles <= 6) {
    var addToInactiveCubes = mutableListOf<Quad<Int, Int, Int, Int>>()
    var addToActiveCubes = mutableListOf<Quad<Int, Int, Int, Int>>()
    minX--
    minY--

    while (w < cycles + 1 ) {

      z = 0
      while (z < cycles + 1) {
      
        var y = minY
        while (y < maxY) {
          var x = minX
          while (x < maxX) {
            val cubes = listOf<Quad<Int, Int, Int, Int>>(
              Quad(x, y, z, w),
              Quad(x, y, -z, -w),
              Quad(x, y, z, -w),
              Quad(x, y, -z, w),
              Quad(x, y, z - 1, -w),
              Quad(x, y, -z, w - 1),
              Quad(x, y, z - 1, w),
              Quad(x, y, z, w - 1)
            )

            for (cube in cubes) {
              val activeNeighborCount = activeCubes.intersect(getNeighbors2(cube) ).size
              if (activeCubes.contains(cube)) {
                if (activeNeighborCount != 2 && activeNeighborCount != 3) {
                  // cube becomes inactive for current z cube
                  addToInactiveCubes.add(cube)
                }  
              } else {
                if (activeNeighborCount == 3) {
                  addToActiveCubes.add(cube)
                  if (y == maxY - 1 || x == maxX - 1) {
                    // increase the y on the fly to make sure we process all of the potential active cubes since the grid is infinite
                    maxY++
                    maxX++
                  }
                }
              }
            }
            x++
          }
          y++
        }
        z++
      }
      w++
    }

    activeCubes.removeAll(addToInactiveCubes)
    inactiveCubes.addAll(addToInactiveCubes)
    inactiveCubes.removeAll(addToActiveCubes)
    activeCubes.addAll(addToActiveCubes)
    activeCubes = activeCubes.toSet().toMutableList()
    inactiveCubes = inactiveCubes.toSet().toMutableList()
  
    w = 0
    cycles++
  }

  // debug2(maxX, maxY, cycles + 1, activeCubes)
  return activeCubes.size
}

private fun getNeighbors2(cube :Quad<Int, Int, Int, Int>) :List<Quad<Int, Int, Int, Int>> {
  var neighbors = mutableListOf<Quad<Int, Int, Int, Int>>(cube)
  for (x in cube.first - 1 until cube.first + 2) {
    for (y in cube.second - 1 until cube.second + 2) {
      for (z in cube.third - 1 until cube.third + 2) {
        for (w in cube.fourth - 1 until cube.fourth + 2) {
          neighbors.add(Quad(x, y, z, w))
        }
      }
    }
  }

  return neighbors.toSet().minusElement(cube).toList()
}

private fun getNeighbors(cube :Triple<Int, Int, Int>) :List<Triple<Int, Int, Int>> {
    var neighbors = mutableListOf<Triple<Int, Int, Int>>(cube)
    for (x in cube.first - 1 until cube.first + 2) {
      for (y in cube.second - 1 until cube.second + 2) {
        for (z in cube.third - 1 until cube.third + 2) {
          neighbors.add(Triple(x, y, z))
        }
      }
    }
    return neighbors.toSet().minusElement(cube).toList()
}

private fun debug2(xMax :Int, yMax :Int, zMax :Int, activeCubes :List<Quad<Int, Int, Int, Int>>) {
  for (z in -(zMax) until zMax) {
    for (w in -(zMax) until zMax) {
    println("z = " + z + "    w = " + w)

    for(y in -yMax until yMax) {
      for (x in -xMax until xMax) {
        if (activeCubes.contains(Quad(x, y, z, w))) {
          print("#")
        } else {
          print(".")
        }
      }
      println("  " + y)
    }

    println()
  }
}
}

private fun debug(xMax :Int, yMax :Int, zMax :Int, activeCubes :List<Triple<Int, Int, Int>>) {
  for (z in -(zMax - 1) until zMax) {
    println("z = " + z )

    for(y in -yMax until yMax) {
      for (x in -xMax until xMax) {
        if (activeCubes.contains(Triple(x, y, z))) {
          print("#")
        } else {
          print(".")
        }
      }
      println("  " + y)
    }

    println()
  }
}