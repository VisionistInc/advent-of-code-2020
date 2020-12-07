import java.io.File;

val yourBag = "shiny gold"
val stopBag = "no other"
val outterBag = "contain"
var total = 0

fun main(args : Array<String>) {
    val input = File(args.first()).readLines()
    println("Solution 1: ${solution1(input)}")
    println("Solution 2: ${solution2(input)}")
}

private fun solution1(bagRules :List<String>) :Int {
   return canContainBag(yourBag, bagRules.filter{ !it.startsWith(yourBag) }, mutableSetOf<String>()).size
}

private fun solution2(bagRules :List<String>) :Int {
     countBagsInside(yourBag, bagRules, 1)
     return total
}

private fun canContainBag(bag :String, bagRules :List<String>, containingBags :MutableSet<String>) :Set<String> {
    for (rule in bagRules) {
        if (rule.contains(bag)) {
            val containingBag = rule.substring(0, rule.indexOf("bags")).trim()
            if (containingBag.equals(bag)) {
                containingBags.add(containingBag)
            }

            if (!containingBags.contains(containingBag)) {
                canContainBag(containingBag, bagRules, containingBags)
            }
        }
    }
    return containingBags
} 

private fun countBagsInside(bag :String, bagRules :List<String>, containingCount :Int) {
    val rule = bagRules.filter{ it.startsWith(bag) }.first()
    var bagCount = 0

    if (!rule.contains(stopBag)) { 
        val ruleParts = rule.substring(rule.indexOf(outterBag) + outterBag.length + 1).split(" ")
        var numBags = 0

        while (numBags < ruleParts.size) {
            val newBagCount = ruleParts.get(numBags).toInt()
            val newBag = ruleParts.get(numBags + 1).plus(" ").plus(ruleParts.get(numBags + 2))
            bagCount = (newBagCount * containingCount) + bagCount
            countBagsInside(newBag, bagRules, newBagCount * containingCount)
            numBags += 4
        }
        total += bagCount
    }

}