package com.illesguy.aoc.common

import scala.io.Source

object Utils {
  def getInputForDay(yr: Int, day: Int): Seq[String] = Source.fromFile(s"../inputs/$yr/day$day.txt")
    .getLines().toSeq.map(_.stripLineEnd)
}
