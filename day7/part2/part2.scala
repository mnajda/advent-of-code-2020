import scala.io.Source
import scala.collection.immutable.Map

object Part2 {
  def parse(line: String) = {
    val (item, rest) = line.split(" bags contain ").splitAt(1)

    if (!line.contains(" no other bags.")) {
      val elems = rest
        .map(
          _.split(",")
            .map(_.trim)
            .map(_.replace(".", ""))
            .map(_.split(" "))
            .map(elem => {
              val count = elem.head.toInt
              val color = elem.slice(1, 3).mkString(" ")

              (count, color)
            })
        )
        .flatten

      (item.head, elems)
    } else {
      (item.head, Array[(Int, String)]())
    }
  }

  def loadFile(path: String): Map[String, Array[(Int, String)]] = {
    val file = Source.fromFile(path)
    val output = file.getLines().map(parse).toMap

    file.close

    output
  }

  def solve(input: Map[String, Array[(Int, String)]]) = {
    def solve(
        input: Map[String, Array[(Int, String)]],
        item: String
    ): Int = {
      val elems = input.getOrElse(item, Array.empty)

      elems
        .map({
          case (count, bag) => count + (count * solve(input, bag))
          case _            => 0
        })
        .sum
    }

    solve(input, "shiny gold")
  }

  def main(args: Array[String]) = {
    val input = loadFile(args(0))
    val result = solve(input)

    println(result)
  }
}
