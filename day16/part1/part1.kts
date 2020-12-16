import java.io.File

typealias Rule = Pair<Int, Int>
typealias Rules = Pair<Rule, Rule>

fun readFile(path: String): List<List<String>> {
    return File(path).readText().split("\n\n").map { line -> line.trim().split("\n") }
}

fun getValues(rule: String): Rule {
    val rules = rule.split("-").map { elem-> elem.toInt() }
    return Rule(rules[0], rules[1])
}

fun makeRules(line: List<String>): Rules {
    val first = getValues(line[0])
    val second = getValues(line[2])
    return Rules(first, second)
}

fun isAnyValid(value: Int, rules: List<Rules>): Boolean {
    fun isValid(value: Int, rule: Rules): Boolean {
        return (value in rule.first.first..rule.first.second) or (value in rule.second.first..rule.second.second)
    }

    return rules.any { rule -> isValid(value, rule) }
}

val contents = readFile(args[0])

val rules = contents[0].map { line -> makeRules(line.split(":")[1].trim().split(" ")) }
val nearbyTickets = contents[2].drop(1).flatMap { line -> line.split(",").map { value -> value.toInt() } }

val result = nearbyTickets.fold(0) { acc, value -> if (isAnyValid(value, rules)) { acc } else { acc + value } }

println(result)
