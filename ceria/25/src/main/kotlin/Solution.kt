import java.io.File

val divideBy = 20201227
val subject = 7


fun main(args : Array<String>) {
  val input = File(args.first()).readLines()
  println("Solution 1: ${solution1(input)}")
  println("Solution 2: ${solution2(input)}")
}

private fun solution1(input :List<String>) :Long {
  var cardPublicKey = input.first().toInt()
  var cardLoopSize = 0
  var transformCardValue = 1
  while (transformCardValue != cardPublicKey) {
    transformCardValue *= subject
    transformCardValue = transformCardValue.rem(divideBy)
    cardLoopSize++
  }

  var doorPublicKey = input.last().toInt()
  var doorLoopSize = 0
  var transformDoorValue = 1
  while (transformDoorValue != doorPublicKey) {
    transformDoorValue *= subject
    transformDoorValue = transformDoorValue.rem(divideBy)
    doorLoopSize++
  }

  var encryptionLoopSize = 0
  var encryptionKey = 1L
  while (encryptionLoopSize != cardLoopSize) {
    encryptionKey *= doorPublicKey
    encryptionKey = encryptionKey.rem(divideBy)
    encryptionLoopSize++
  }

  return encryptionKey
}

private fun solution2(input :List<String>) :Long {
  return 0
}