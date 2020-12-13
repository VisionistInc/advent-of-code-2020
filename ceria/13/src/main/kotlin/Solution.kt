import java.io.File
import java.math.BigInteger

fun main(args : Array<String>) {
    val input = File(args.first()).readLines()
    println("Solution 1: ${solution1(input)}")
    println("Solution 2: ${solution2(input)}")
}

private fun solution1(input :List<String>) :Int {
  var time = input.first().toInt()
  val busses = input.last().split(",").filter{ it != "x" }.map{ it.toInt() }
  var yourBus = 0

  while (yourBus == 0) {
    for (bus in busses) {
      if (time.rem(bus) == 0) {
        yourBus = bus
        break
      }
    }

    if (yourBus == 0) {
      time++
    }
  }

  return (time - input.first().toInt()) * yourBus
}

private fun solution2(input :List<String>) :Long {
  val busses = input.last().split(",")
  val vals = mutableListOf<Long>()
  val modVals = mutableListOf<Long>()

  for (i in busses.indices) {
    if (! busses[i].equals("x")) {
      modVals.add(busses[i].toLong())
      vals.add(i.toLong())
    }
  }
  
  return modVals.product() - chineseRemainder(vals, modVals)
}

private fun chineseRemainder(vals : List<Long>, modVals : List<Long>) : Long{
  val modProduct = modVals.product()
  var res = 0L
  for(i in vals.indices){
      val gcd = extendedEuclid(modVals[i], modProduct/modVals[i])
      val a = BigInteger(vals[i].toString())
      val b = BigInteger(gcd[2].toString())
      val c = BigInteger((modProduct/modVals[i]).toString())
      val mod = BigInteger(modProduct.toString())
      res += a.multiply(b).multiply(c).remainder(mod).toLong()
      res = (res.rem(modProduct) + modProduct).rem(modProduct)
  }
  return res
}

/**
 * Find the product of all the numbers in the list
 */
private fun <E : Number> List<E>.product(): Long {
  if (this.isEmpty()) {
      return 0L
  }
  var product = 1L
  for (numb in this) {
      when (numb) {
          is Long -> product *= numb
      }
  }
  return product
}

private fun extendedEuclid(a: Long, b: Long): LongArray {
  return if (b == 0L) {
      longArrayOf(a, 1L, 0L)
  } else {
      val res = extendedEuclid(b, a.rem(b))
      longArrayOf(res[0], res[2], res[1] - (a / b) * res[2])
  }
}