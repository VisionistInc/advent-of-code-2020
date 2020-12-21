import java.io.File

val allergenStart ="(contains "

fun main(args : Array<String>) {
  val input = File(args.first()).readLines()
  println("Solution 1: ${solution1(input)}")
  println("Solution 2: ${solution2(input)}")
}

private fun solution1(input :List<String>) :Int {
  var allergenToCandidates = mutableMapOf<String, MutableSet<String>>()
  var ingredientsLists = mutableListOf<List<String>>()

  for (line in input) {
    if (line.contains(allergenStart)) {
      val allergens = line.substring(line.indexOf(allergenStart) + allergenStart.length, line.lastIndexOf(")")).split(", ")
      for (allergen in allergens) {
        val ingredients = line.substring(0, line.indexOf(allergenStart) - 1).split(" ").toSet()
        if (allergenToCandidates.contains(allergen)) {
          var existingIngredients = allergenToCandidates.get(allergen)!!
          allergenToCandidates.put(allergen, existingIngredients.intersect(ingredients).toMutableSet())
        } else {
          allergenToCandidates.put(allergen, ingredients.toMutableSet())
        }
      }
      ingredientsLists.add(line.substring(0, line.indexOf(allergenStart) - 1).split(" "))
    } else {
      ingredientsLists.add(line.split(" "))
    }
  }

  var keepFiltering = true
  while (keepFiltering)  {
    for ((k, v) in allergenToCandidates) {
      if (v.size == 1) {
        var known = v.first()
        for ((_, v2) in allergenToCandidates.minus(k)) {
          if (v2.contains(known)) {
            v2.remove(known)
          }
        }
      }
    }

    for (v in allergenToCandidates.values) {
      if (v.size > 1) {
        break
      }
      keepFiltering = false
    }
  }

  return ingredientsLists.map{ it.minus(allergenToCandidates.values.filter{ !it.isEmpty() }.map{ it.first() } )}.flatten().size
}

private fun solution2(input :List<String>) :String {
  var allergenToCandidates = mutableMapOf<String, MutableSet<String>>()

  for (line in input) {
    if (line.contains(allergenStart)) {
      val allergens = line.substring(line.indexOf(allergenStart) + allergenStart.length, line.lastIndexOf(")")).split(", ")
      for (allergen in allergens) {
        val ingredients = line.substring(0, line.indexOf(allergenStart) - 1).split(" ").toSet()
        if (allergenToCandidates.contains(allergen)) {
          var existingIngredients = allergenToCandidates.get(allergen)!!
          allergenToCandidates.put(allergen, existingIngredients.intersect(ingredients).toMutableSet())
        } else {
          allergenToCandidates.put(allergen, ingredients.toMutableSet())
        }
      }
    }
  }

  var keepFiltering = true
  while (keepFiltering)  {
    for ((k, v) in allergenToCandidates) {
      if (v.size == 1) {
        var known = v.first()
        for ((_, v2) in allergenToCandidates.minus(k)) {
          if (v2.contains(known)) {
            v2.remove(known)
          }
        }
      }
    }

    for (v in allergenToCandidates.values) {
      if (v.size > 1) {
        break
      }
      keepFiltering = false
    }
  }

  var sorted = allergenToCandidates.keys.sortedBy { it }
  var dangerous = mutableListOf<String>()
  for (k in sorted) {
    dangerous.add(allergenToCandidates.get(k)!!.first())
  }

  return dangerous.joinToString(separator = ",")
}