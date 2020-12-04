import java.io.File


val requiredFields = listOf("byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid")
val optionalField = "cid"
val validUnits = mapOf<String, IntRange>("cm" to IntRange(150, 193), "in" to IntRange(59, 76)) 
val validEyeColors = listOf("amb", "blu", "brn", "gry", "grn", "hzl", "oth")

fun main(args : Array<String>) {
    val input = File(args.first()).readLines()
    println("Solution 1: ${solution1(detectPassports(input))}")
    println("Solution 2: ${solution2(detectPassports(input))}")
}

// 254
private fun solution1(input: List<String>) :Int {
    var validPassportCount = 0
    for ( p in input ) {
        val fieldsNotInPassport = requiredFields.minus(p.split(" ").map{field -> field.split(":").get(0) })
        if (fieldsNotInPassport.isEmpty() || (fieldsNotInPassport.size == 1 && fieldsNotInPassport.contains(optionalField)) ) {
            validPassportCount++
        }
    }
	return validPassportCount
}

// 185 == too high
private fun solution2(input: List<String>) :Int {
    var validPassportCount = 0
    for ( p in input ) {
        val fieldsInPassport = p.split(" ").map{field -> field.split(":").get(0) }
        val fieldsNotInPassport = requiredFields.minus( fieldsInPassport )
        if (fieldsNotInPassport.isEmpty() || (fieldsNotInPassport.size == 1 && fieldsNotInPassport.contains(optionalField)) ) {
            val fieldValues = p.split(" ").associate { 
                val (left, right) = it.split(":")
                left to right 
            }

            var valid = true
            for (f in requiredFields) {
                val value = fieldValues.get(f)
                when (f) {
                    "byr" -> {
                        if (value?.length != 4 || value.filter{ it.isDigit() }.length != 4 || !IntRange(1920, 2002).contains(value.toInt())) {
                            valid = false
                        }
                    }
                    "iyr" -> {
                        if (value?.length != 4 || value.filter{ it.isDigit() }.length != 4 || !IntRange(2010, 2020).contains(value.toInt())) {
                            valid = false
                        }
                    }
                    "eyr" -> {
                        if (value?.length != 4 || value.filter{ it.isDigit() }.length != 4 || !IntRange(2020, 2030).contains(value.toInt())) {
                            valid = false
                        }
                    }
                    "hgt" -> {
                        if ( !validUnits.containsKey(value?.takeLast(2)) || validUnits.get(value?.takeLast(2))?.contains(value?.dropLast(2)?.toInt()) == false ) {
                            valid = false
                        } 
                    }
                    "hcl" -> {
                        if (value?.length != 7 || value.first() != '#' || 
                            value.takeLast(6).filter{ it in 'a'..'f' || it in '0'..'9' }.length != 6) {
                                valid = false
                            }
                    }
                    "ecl" -> {
                        if (value?.length != 3 || !validEyeColors.contains(value)) {
                            valid = false
                        }
                    }
                    "pid" -> {
                        if (value?.length != 9 || value.filter{ it.isDigit() }.length != 9) {
                            valid = false
                        }
                    }
                }
                if (!valid) {
                    break
                }
            }

            if (valid) {
                validPassportCount++
            }
        }
    }
	return validPassportCount
}

private fun detectPassports(input :List<String>) :List<String> {
    var passports = mutableListOf<String>()
    var completePassport = ""
    for (line in input) {
     	if (line.isNullOrBlank()) {
            passports.add(completePassport.trim())
            completePassport = ""
            continue
        }
        
        completePassport = completePassport.plus(line).plus(" ")
    }
    
    passports.add(completePassport.trim())
    return passports
}