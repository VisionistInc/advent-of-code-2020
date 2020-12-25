import java.io.File

fun main(args : Array<String>) {
  val input = File(args.first()).readLines()
  println("Solution 1: ${solution1(input)}")
  println("Solution 2: ${solution2(input)}")
}

private fun solution1(input :List<String>) :Int {
  var rulesMap = mutableMapOf<String, String>()
  val rules = input.subList(0, input.indexOf(""))
  var messages = input.subList(input.indexOf("") + 1, input.size)
  
  for (rule in rules) {
    val ruleNum = rule.substring(0, rule.indexOf(":"))
    val ruleVal = rule.substring(rule.indexOf(" ") + 1, rule.length)
    if (ruleVal.contains('|')) {
      var stringified = "( ".plus(ruleVal.substring(0, ruleVal.indexOf("|") -1)).plus(" ) | ( ").plus(ruleVal.substring(ruleVal.indexOf("|") + 2 ) ).plus(" )")
      rulesMap.put(ruleNum, "( ".plus(stringified).plus(" )"))
    } else {
      rulesMap.put(ruleNum, "( ".plus(ruleVal.replace("\"", "")).plus(" )"))
    }
  }

  val pattern = "^".plus(buildRegex(rulesMap)).plus("$").toRegex()
  return messages.filter{ pattern.containsMatchIn(it) }.size
}

// 348 == too low  450 == too high
private fun solution2(input :List<String>) :Int {
  var rulesMap = mutableMapOf<String, String>()
  val rules = input.subList(0, input.indexOf(""))
  var messages = input.subList(input.indexOf("") + 1, input.size)
  
  for (rule in rules) {
    val ruleNum = rule.substring(0, rule.indexOf(":"))
    val ruleVal = rule.substring(rule.indexOf(" ") + 1, rule.length)
    if (ruleVal.contains('|')) {
      var stringified = "( ".plus(ruleVal.substring(0, ruleVal.indexOf("|") -1)).plus(" ) | ( ").plus(ruleVal.substring(ruleVal.indexOf("|") + 2 ) ).plus(" )")
      rulesMap.put(ruleNum, "( ".plus(stringified).plus(" )"))
    } else {
      rulesMap.put(ruleNum, "( ".plus(ruleVal.replace("\"", "")).plus(" )"))
    }
  }

  // Manually modify the rules
  // 8: 42   becomes
  // 8: 42 | 42 8  which is essentially (42)+ (1 to n times)
  // rulesMap.put("8", rulesMap.get("8").plus(" | ( 42 ( 42 + ) )"))
  rulesMap.put("8", rulesMap.get("8").plus(" | ( ( 42 ) {2,} )"))

  // 11: 42 31    becomes
  // 11: 42 31 | 42 11 31   which is (42+ 31)
  // rulesMap.put("11", rulesMap.get("11").plus( " | ( 42 ( 42 + ) 31 )"))
  rulesMap.put("11", "( ( 42 * ) 42 31 )")

  var pattern = "^".plus(buildRegex(rulesMap)).plus("$").toRegex()
// println(pattern)

  // var filteredMessages = messages.filter{ pattern.containsMatchIn(it) }
  // for (message in filteredMessages ) {
  //   val match = pattern.find(message)
  //   if (!message.equals(match?.value)) {
  //     println("-------------------------------------------     " + message)
  //   }
  //     // println(match?.value)
  
  // }

  // println("*****************************************************")
  // println("*****************************************************")


  // messages.filter{ pattern.containsMatchIn(it) }.map{ println(it) }

  return messages.filter{ pattern.containsMatchIn(it) }.size
}

private fun buildRegex(rules :Map<String, String>) :String {
  var rulesMap = rules.toMutableMap()
  while ( !allLetters(rulesMap.get("0")!!) ) {
    for (rule in findReplacingRules(rulesMap)) {
      var ruleVal = rulesMap.get(rule)!!
      for ( (k, v) in rulesMap) {

        // Split into tokens to make sure it's the full rule, and not partial, i.e. that it's rule "8" and not "88"
        var parts = v.split(" ").toMutableList()
        for (i in parts.indices) {
          if (parts[i].equals(rule)) {
            parts[i] = ruleVal
          }
        }

        rulesMap.put(k, parts.joinToString(separator = " "))
      }
    }

    // println("+++++++++++++++++++++++++++++")
  }
  return rulesMap.get("0")!!.replace(" ", "")
}

private fun findReplacingRules(rulesMap :Map<String, String>) :Set<String> {
  var replacing = mutableSetOf<String>()
  for ((rKey, rVal) in rulesMap) {
    if (allLetters(rVal)) {
      replacing.add(rKey)
    }
  }
  return replacing
}

private fun allLetters(test :String) :Boolean {
  var allLetters = true
  for (v in test.split(" ")) {
    if (!v.trim().equals("a") && !v.trim().equals("b") && !v.trim().equals("(")
        && !v.trim().equals(")") && !v.trim().equals("|") && !v.trim().equals("+")
        && !v.trim().equals("?") && !v.trim().equals("*") && !v.trim().equals("{2,}") ) {
      allLetters = false
      break
    }
  }
  return allLetters
}