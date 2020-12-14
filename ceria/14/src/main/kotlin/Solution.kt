import java.io.File

fun main(args : Array<String>) {
    val input = File(args.first()).readLines()
    println("Solution 1: ${solution1(input)}")
    println("Solution 2: ${solution2(input)}")
}

private fun solution1(input :List<String>) :Long {
  var memory = mutableMapOf<Int, Long>()
  var mask = StringBuilder()

  for (line in input) {
    if (line.startsWith("mask")) {
      mask = StringBuilder()
      mask.append( line.substring(line.lastIndexOf(' ')).trim() )
    } else {
      val address = line.substring(line.indexOf('[') + 1, line.indexOf(']') ).toInt()
      val decimal = line.substring(line.lastIndexOf(' ') + 1).toInt()
      val binary = Integer.toBinaryString(decimal).padStart(36, '0')
      val masked = applyMask(mask.toString(), binary)
      val backToDecimal = binaryToDecimal(masked)
      memory.put(address, backToDecimal)
    }
  }

  return memory.values.sum()
}

private fun solution2(input :List<String>) :Long {
  var memory = mutableMapOf<Long, Long>()
  var mask = StringBuilder()

  for (line in input) {
    if (line.startsWith("mask")) {
      mask = StringBuilder()
      mask.append( line.substring(line.lastIndexOf(' ')).trim() )
    } else {
      val address = line.substring(line.indexOf('[') + 1, line.indexOf(']') ).toInt()
      val value = line.substring(line.lastIndexOf(' ') + 1).toInt()
      val binary = Integer.toBinaryString(address).padStart(36, '0')

      val masked = applyMaskV2(mask.toString(), binary)
      val floatingMasks = generateFloatingMasks(masked)
      for (fm in floatingMasks) {
        memory.put(binaryToDecimal(fm), value.toLong())
      }
 
      // memory.put(address, backToDecimal)
    }
  }

  return memory.values.sum()
}

private fun applyMaskV2(mask :String, binary :String) :String {
  var newBinary = StringBuilder()
  for (i in mask.indices) {
    if (mask[i].equals('0')) {
      newBinary.append(binary[i])
    } else if (mask[i].equals('1')) {
      newBinary.append('1')
    } else {
      newBinary.append('X')
    }
  }
  return newBinary.toString()
}

private fun generateFloatingMasks(mask :String) :List<String> {
  var floatingMasks = mutableListOf<CharArray>(mask.toCharArray())
  var xIndexes = mutableListOf<Int>()
  mask.forEachIndexed { index, c ->
    if (c.equals('X') ) xIndexes.add(index)
   }

   for (x in xIndexes) {
     var copyMasks = mutableListOf<CharArray>()
     for (fMask in floatingMasks) {
        val copy0 = fMask.clone()
        copy0[x] = '0'
        copyMasks.add(copy0)
        val copy1 = fMask.clone()
        copy1[x] = '1'
        copyMasks.add(copy1)
     }
     floatingMasks = copyMasks
   }

  return floatingMasks.map{ String(it) }
}

private fun applyMask(mask :String, binary :String) :String {
  var newBinary = StringBuilder()
  for (i in mask.indices) {
    if (!mask[i].equals('X')) {
      newBinary.append(mask[i])
    } else {
      newBinary.append(binary[i])
    }
  }
  return newBinary.toString()
}

private fun binaryToDecimal(binaryNumber :String) :Long {
  var sum = 0L
  binaryNumber.reversed().forEachIndexed {
      k, v -> sum += v.toString().toLong() * Math.pow(2.0, k.toDouble()).toLong()
  }
  return sum
}
