import java.io.File
import java.util.Queue
import java.util.LinkedList

fun main(args : Array<String>) {
    val input = File(args.first()).readLines()
    println("Solution 1: ${solution1(input)}")
    println("Solution 2: ${solution2(input)}")
  }

  private fun solution1(input :List<String>) :Int {
    var p1: Queue<Int> = LinkedList<Int>(input.subList(1, input.indexOf("")).map { it.toInt() }) 
    var p2: Queue<Int> = LinkedList<Int>(input.subList(input.indexOf("") + 2, input.size).map { it.toInt() })

    while (!p1.isEmpty() && !p2.isEmpty()) {
      var c1 = p1.remove()
      var c2 = p2.remove()

      if (c1 > c2) {
        p1.add(c1)
        p1.add(c2)
      } else {
        p2.add(c2)
        p2.add(c1)
      }
    }

    val winner = if (p1.isEmpty()) p2 else p1
    var productSum = 0
    for (i in winner.size downTo 1) {
      productSum += winner.remove() * i
    }

    return productSum
  }

  private fun solution2(input :List<String>) :Int {
    var p1: Queue<Int> = LinkedList<Int>(input.subList(1, input.indexOf("")).map { it.toInt() }) 
    var p2: Queue<Int> = LinkedList<Int>(input.subList(input.indexOf("") + 2, input.size).map { it.toInt() })

    var finalDecks = recurseCombat(p1, p2)
    var winner = if (finalDecks.first.isEmpty()) finalDecks.second else finalDecks.first
    var productSum = 0
    for (i in winner.size downTo 1) {
      productSum += winner.remove() * i
    }

    return productSum
  }

  private fun recurseCombat(player1 :Queue<Int>, player2 :Queue<Int>) :Pair<Queue<Int>, Queue<Int>> {
    var p1 = player1
    var p2 = player2
    var previousRounds = mutableListOf<Pair<Queue<Int>, Queue<Int>>>()

    while (!p1.isEmpty() && !p2.isEmpty()) {
      if (previousRounds.contains(Pair(p1, p2))) {
        // sudden death - p1 wins
        p1.add(-1)
        p2.add(-1)
        break
      }

      // play on!
      val p1C: Queue<Int> = LinkedList<Int>(p1.toIntArray().copyOf().toList())
      val p2C: Queue<Int> = LinkedList<Int>(p2.toIntArray().copyOf().toList())
      previousRounds.add(Pair(p1C, p2C))
      var c1 = p1.remove()
      var c2 = p2.remove()

      if (p1.size >= c1 && p2.size >= c2) {
        var p1Copy: Queue<Int> = LinkedList<Int>(p1.toIntArray().copyOf(c1).toList())
        var p2Copy: Queue<Int> = LinkedList<Int>(p2.toIntArray().copyOf(c2).toList())
        var decks = recurseCombat(p1Copy, p2Copy)

        if (decks.first.contains(-1) && decks.second.contains(-1)) {
          // both decks contain the -1 marker, so p1 automatically wins this round
          p1.add(c1)
          p1.add(c2)
        } else {
          if (decks.first.isEmpty()) {
            p2.add(c2)
            p2.add(c1)
          } else {
            p1.add(c1)
            p1.add(c2)
          }
        }
        
      } else {
        if (c1 > c2) {
          p1.add(c1)
          p1.add(c2)
        } else {
          p2.add(c2)
          p2.add(c1)
        }
      }
    }

    return Pair(p1, p2)
  }