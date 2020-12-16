import java.io.File
import java.util.Collections

fun main(args : Array<String>) {
    val input = File(args.first()).readLines()
    println("Solution 1: ${solution1(input)}")
    println("Solution 2: ${solution2(input)}")
}

private fun solution1(input :List<String>) :Int {
  val rules = input.subList(0, input.indexOf("") )
  val nearbyTickets = input.subList(input.lastIndexOf("nearby tickets:") + 1, input.size).map{ it.split(',').map{ it.toInt() }}

  var rulesMap = mutableMapOf<String, Pair<IntRange, IntRange>>()
  for (r in rules) {
    val name = r.substring(0, r.indexOf(':'))
    val firstRange = r.substring(r.indexOf(":") + 2, r.indexOf(" or ")).split('-')
    val secondRange = r.substring(r.lastIndexOf(" ") + 1).split('-')
   rulesMap.put(name, Pair(IntRange(firstRange[0].toInt(), firstRange[1].toInt()), IntRange(secondRange[0].toInt(), secondRange[1].toInt())))
  }

  var invalidValues = mutableListOf<Int>()
  for (ticket in nearbyTickets) {
    for (num in ticket) {
      
      var valid = false 
      for (pair in rulesMap.values) {
        if (pair.first.contains(num) || pair.second.contains(num)) {
          valid = true
          break
        }
      }

      if (!valid) invalidValues.add(num)
    }
  }

  return invalidValues.sum()
}

private fun solution2(input :List<String>) :Long {
  val rules = input.subList(0, input.indexOf("") )
  val yourTicket = input.get(input.indexOf("your ticket:") + 1).split(',').map{ it.toInt() }
  val nearbyTickets = input.subList(input.lastIndexOf("nearby tickets:") + 1, input.size).map{ it.split(',').map{ it.toInt() }}

  var rulesMap = mutableMapOf<String, Pair<IntRange, IntRange>>()
  for (r in rules) {
    val name = r.substring(0, r.indexOf(':'))
    val firstRange = r.substring(r.indexOf(":") + 2, r.indexOf(" or ")).split('-')
    val secondRange = r.substring(r.lastIndexOf(" ") + 1).split('-')
   rulesMap.put(name, Pair(IntRange(firstRange[0].toInt(), firstRange[1].toInt()), IntRange(secondRange[0].toInt(), secondRange[1].toInt())))
  }

  var invalidTickets = mutableListOf<List<Int>>()
  for (ticket in nearbyTickets) {
    for (num in ticket) {
      
      var valid = false 
      for (pair in rulesMap.values) {
        if (pair.first.contains(num) || pair.second.contains(num)) {
          valid = true
          break
        }
      }

      if (!valid) {
        invalidTickets.add(ticket)
        break
      }
    }
  }

  val validTickets = nearbyTickets.minus(invalidTickets)
  var validPositionsMap = mutableMapOf<String, List<Int>>()
  rulesMap.forEach {rule, valuePair ->
    validPositionsMap.put(rule, (0..rules.size - 1).toList() )
    for (ticket in validTickets) {
      for (i in ticket.indices) {
        if ( !valuePair.first.contains(ticket[i]) && !valuePair.second.contains(ticket[i])) {
          if (validPositionsMap.get(rule)!!.contains(i)) {
              var pos = validPositionsMap.get(rule)!!.toMutableList()
              pos.remove(i)
              validPositionsMap.put(rule, pos)
          }
        }
      }
    }
  }

  var ruleToPosition = mutableMapOf<String, Int>()
  while (ruleToPosition.size != validPositionsMap.size) {
    var rulesToRemove = mutableListOf<String>()
    validPositionsMap.forEach {rule, indexes ->
      if (indexes.size == 1) {
        ruleToPosition.put(rule, indexes.get(0))
        rulesToRemove.add(rule)
      }
    }
    for (rule in rulesToRemove) {
      val indexToRemove = validPositionsMap.get(rule)!!.get(0)
      validPositionsMap.forEach {r, indexes ->
        if (indexes.contains(indexToRemove)) {
          val newIndexes = indexes.toMutableList()
          newIndexes.remove(indexToRemove)
          validPositionsMap.put(r, newIndexes)
        }
      }
    }
  }

  var departureIndexes = mutableListOf<Int>()
  ruleToPosition.forEach {rule, pos ->
    if (rule.startsWith("departure")) {
      departureIndexes.add(pos)
    }
  }

  var product = 1L
  for (i in departureIndexes) {
    product *= yourTicket.get(i)
  }

  return product
}