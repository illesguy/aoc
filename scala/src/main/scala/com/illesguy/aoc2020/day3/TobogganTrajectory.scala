package com.illesguy.aoc2020.day3

import com.illesguy.aoc.common.Utils

object TobogganTrajectory extends App {
  def getInfiteListElement[T](list: Seq[T], index: Int): T = list(index % list.size)
  
  val trajectoryMap: Seq[Seq[Int]] = Utils.getInputForDay(2020, 3).map { row =>
    row.toCharArray.map {
      case '#' => 1
      case _ => 0
    }.toSeq
  }
  
  def getTreesOnWay(tMap: Seq[Seq[Int]], downStep: Int, rightStep: Int): Int = {
    val rowsHit = downStep until tMap.size by downStep
    val fieldsHit = rowsHit.map { row =>
      val col = (row / downStep) * rightStep
      getInfiteListElement(tMap(row), col)
    }
    fieldsHit.sum
  }
  
  val trajectory1 = getTreesOnWay(trajectoryMap, 1, 1).toLong
  val trajectory2 = getTreesOnWay(trajectoryMap, 1, 3).toLong
  val trajectory3 = getTreesOnWay(trajectoryMap, 1, 5).toLong
  val trajectory4 = getTreesOnWay(trajectoryMap, 1, 7).toLong
  val trajectory5 = getTreesOnWay(trajectoryMap, 2, 1).toLong
  
  println(trajectory1)
  println(trajectory2)
  println(trajectory3)
  println(trajectory4)
  println(trajectory5)
  println(trajectory1 * trajectory2 * trajectory3 * trajectory4 * trajectory5)
}
