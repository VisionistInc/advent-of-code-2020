import java.io.File

fun main(args : Array<String>) {
  val input = File(args.first()).readLines()
  println("Solution 1: ${solution1(input)}")
  println("Solution 2: ${solution2(input)}")
}

private fun solution1(input :List<String>) :Long {
  var total = 0L
  for (line in input) {
    total += evalExpression(line, true)
  }
  return total
}

private fun solution2(input :List<String>) :Long {
  var total = 0L
  for (line in input) {
    total += evalExpression(line, false)
  }
  return total
}

private fun evalExpression(line :String, normalPrecedence :Boolean) : Long {
  var exp = StringBuilder(line)

  while (exp.contains("(")) {
    var openParen = exp.indexOf("(")
    var closeParen = exp.indexOf(")")
    while (exp.substring(openParen + 1, closeParen).contains("(")) {
      openParen = exp.indexOf("(", openParen + 1)
    }
    
    val evaluated = eval(exp.substring( openParen + 1, closeParen).trim(), normalPrecedence).toString()
    exp = StringBuilder(exp.replaceRange(openParen, closeParen + 1, evaluated))
  }

  return eval(exp.toString(), normalPrecedence)
}

private fun eval(exp :String, normalPrecedence :Boolean) :Long {
  var left = 0L
  if (normalPrecedence) {
    val parts = exp.split(" ")
    left = parts[0].toLong()
    for (n in 1..parts.size - 1 step 2) {
      when (parts[n]) {
        "+" -> { left += parts[n + 1].toLong() }
        "*" -> { left *= parts[n + 1].toLong() }
      }
    }
  } else {
    var newExp = StringBuilder(exp)

    while (newExp.contains("+")) {
      val parts = newExp.split(" ")  
      newExp = StringBuilder()
      for (n in 1..parts.size - 1 step 2) {
        if (parts[n].equals("+")) {
          var sum = parts[n - 1].toLong() + parts[n + 1].toLong()
          newExp.append(sum)
          newExp.append(" ")
          for (i in n + 2..parts.size - 1) {
            newExp.append(parts[i])
            newExp.append(" ")
          }
          break
        } else {
            newExp.append(parts[n - 1])
            newExp.append(" ")
            newExp.append(parts[n])
            newExp.append(" ")
        }
      }
    }

    if (newExp.contains("*")) {
      val parts = newExp.toString().trim().split(" ")
      left = parts[0].toLong()
      for (n in 1..parts.size - 1 step 2) {
        left *= parts[n + 1].toLong() 
      }
    } else {
      left = newExp.toString().trim().toLong()
    }
  
  }

  return left
}