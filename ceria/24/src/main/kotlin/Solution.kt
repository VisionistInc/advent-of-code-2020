import java.io.File

val tokenMap = mapOf<String, Pair<Int, Int>>(
  "e" to Pair(2, 0), 
  "w" to Pair(-2, 0),
  "se" to Pair(1, -1), 
  "sw" to Pair(-1, -1),  
  "ne" to Pair(1, 1),
  "nw" to Pair(-1, 1)
)

  fun main(args : Array<String>) {
    val input = File(args.first()).readLines()
    println("Solution 1: ${solution1(input)}")
    println("Solution 2: ${solution2(input)}")
  }

  private fun solution1(input :List<String>) :Int {
    var landingTiles = mutableMapOf<Pair<Int, Int>, Int>()

    for (line in input) {
      val landingTile = findLandingTile(tokenize(line))
      if (landingTiles.contains(landingTile)) {
        var numTimesFlipped = landingTiles.get(landingTile)!!
        landingTiles.put(landingTile, numTimesFlipped + 1)
      } else {
        landingTiles.put(landingTile, 1)        
      }
    }

    return landingTiles.filter{ it.value.rem(2) == 1 }.size
  }

  private fun solution2(input :List<String>) :Int {
    var landingTiles = mutableMapOf<Pair<Int, Int>, Int>()

    for (line in input) {
      val landingTile = findLandingTile(tokenize(line))
      if (landingTiles.contains(landingTile)) {
        var numTimesFlipped = landingTiles.get(landingTile)!!
        landingTiles.put(landingTile, numTimesFlipped + 1)
      } else {
        landingTiles.put(landingTile, 1)        
      }
    }

   var blackTiles = landingTiles.filter{ it.value.rem(2) == 1 }.keys.toMutableSet()
   var day = 0
   while (day < 100) {
     val blackTilesCopy = blackTiles.toSet()
     var whiteTiles = fillInWhiteTiles(blackTilesCopy).toMutableSet()

     for (tile in blackTilesCopy) {
      val adjacentTiles = getAdjacentTiles(tile)
      val adjacentBlackTiles = blackTilesCopy.intersect(adjacentTiles)
      if (adjacentBlackTiles.size == 0 || adjacentBlackTiles.size > 2) {
        // flips to white (remove from black)
        blackTiles.remove(tile)
      }
     }

     for (tile in whiteTiles) {
      val adjacentTiles = getAdjacentTiles(tile)
      val adjacentBlackTiles = adjacentTiles.intersect(blackTilesCopy)
      if (adjacentBlackTiles.size == 2) {
        // flips to black (remove from white)
        blackTiles.add(tile)
      }
     }

     day++
   }
    return blackTiles.size
  }

  private fun tokenize(line :String) :List<String> {
    var tokens = mutableListOf<String>()
    var index = 0
    while (index < line.length) {
      if (line[index] == 'e' || line[index] == 'w') {
        tokens.add(line[index].toString())
        index++
      } else {
        tokens.add(line[index].toString() + line[index + 1].toString())
        index += 2
      }      
    }
    return tokens
  }

  private fun findLandingTile(tokens :List<String>) :Pair<Int, Int> {
    var landingTile = Pair(0, 0)
    
    for (token in tokens) {
     val pos = tokenMap.get(token)!!
     landingTile = Pair(landingTile.first + pos.first, landingTile.second + pos.second)
    }
    
    return landingTile
  }

  private fun fillInWhiteTiles(blackTiles :Set<Pair<Int, Int>>) :Set<Pair<Int, Int>> {
    return blackTiles.map{ getAdjacentTiles(it).minus(blackTiles) }.flatten().toSet()
  }

  private fun getAdjacentTiles(tile :Pair<Int, Int>) :Set<Pair<Int, Int>> {
    return setOf<Pair<Int, Int>>(
      Pair(tile.first + 2, tile.second),
      Pair(tile.first - 2, tile.second),
      Pair(tile.first + 1, tile.second + 1),
      Pair(tile.first + 1, tile.second - 1),
      Pair(tile.first - 1, tile.second + 1),
      Pair(tile.first - 1, tile.second - 1)
    )
  }