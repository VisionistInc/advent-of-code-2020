import java.io.File
import java.util.Collections

fun main(args : Array<String>) {
    val input = File(args.first()).readLines()
    println("Solution 1: ${solution1(input)}")
    println("Solution 2: ${solution2(input)}")
}

private fun solution1(input :List<String>) :Int {
  var nums = input.first().split(",").map{ it.toInt() }.toMutableList()

  while (nums.size < 2020) {
    var lastNumSpoken = nums.last()
    if (Collections.frequency(nums, lastNumSpoken) == 1) {
      nums.add(0)
    } else {
      var lastSpokenIndex = nums.dropLast(1).lastIndexOf(lastNumSpoken)
      nums.add((nums.size - 1) - lastSpokenIndex)
    }

  }

  return nums.last()
}

private fun solution2(input :List<String>) :Int {
  var nums = input.first().split(",").map{ it.toInt() }
  var numsMap = mutableMapOf<Int, Pair<Int, Int>>()   // num => Pair(firstIndex, secondIndex)
  for (n in nums.indices) {
    numsMap.put(nums[n], Pair(n, -1))
  }

  var turn = nums.size
  var lastSpoken = nums.last()
  while (turn < 30000000) {
    if (numsMap.containsKey(lastSpoken)) {
      var indexPair = numsMap.get(lastSpoken)
      if (indexPair!!.second == -1 && indexPair.first == turn - 1) {
          // not spoken before
          lastSpoken = 0
      } else {
        // spoken before
        val lastSpokenPair = numsMap.get(lastSpoken)
        lastSpoken = lastSpokenPair!!.second - lastSpokenPair.first
      }

    } else {
      // not spoken before
      lastSpoken = 0
    }

    if (numsMap.containsKey(lastSpoken)) {
      val lastSpokenPair = numsMap.get(lastSpoken)
      val newFirst = if (lastSpokenPair!!.second == -1) lastSpokenPair.first else lastSpokenPair.second
      var updatedPair = Pair(newFirst, turn)
      numsMap.put(lastSpoken, updatedPair)
    } else {
      numsMap.put(lastSpoken, Pair(turn, -1))
    }

    turn++
  }

  return lastSpoken
}