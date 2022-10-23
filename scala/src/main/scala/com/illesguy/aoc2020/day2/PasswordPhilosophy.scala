package com.illesguy.aoc2020.day2

import com.illesguy.aoc.common.Utils

import scala.util.matching.Regex

object PasswordPhilosophy extends App {
  val pattern: Regex = raw"^(\d+)-(\d+) (\w): (\w+)".r
  
  case class Record(low: Int, high: Int, character: Char, password: String)
  val records = Utils.getInputForDay(2020, 2).map {
    case pattern(l, h, c, p) => Record(l.toInt, h.toInt, c.head, p)
  }
  
  def isValidPwPart1(r: Record): Boolean = {
    val occurences = r.password.count(_ == r.character)
    occurences >= r.low && occurences <= r.high
  }

  def isValidPwPart2(r: Record): Boolean = {
    r.password.lift(r.low - 1).contains(r.character) ^
      r.password.lift(r.high - 1).contains(r.character)
  }
  
  println(records.count(isValidPwPart1))
  println(records.count(isValidPwPart2))
}
