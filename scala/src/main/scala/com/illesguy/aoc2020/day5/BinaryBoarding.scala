package com.illesguy.aoc2020.day5

import com.illesguy.aoc.common.Utils

object BinaryBoarding extends App {
  val inputs: Seq[String] = Utils.getInputForDay(2020, 5)
  
  def codeToNumber(code: String): Int = Integer.parseInt(code.split("").map {
    case "B" | "R" => 1
    case "F" | "L" => 0
  }.mkString(""), 2)

  val seatIds = inputs.map(codeToNumber).toSet
  println(seatIds.max)
  
  val mySeat = (0 until 1024).find { s =>
    !seatIds.contains(s) && seatIds.contains(s - 1) && seatIds.contains(s + 1) 
  }.getOrElse(throw new RuntimeException("No seat for you!"))
  println(mySeat)
}
