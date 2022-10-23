package com.illesguy.aoc2020.day6

import com.illesguy.aoc.common.Utils

object CustomCustoms extends App {
  type PersonAnswers = Set[Char]
  
  val inputs: Seq[Seq[PersonAnswers]] = Utils.getInputForDay(2020, 6).foldLeft(Seq(Seq
    .empty[PersonAnswers])) {
    case (soFar, "") => Seq.empty[PersonAnswers] +: soFar
    case (soFar, inp) => (inp.toCharArray.toSet +: soFar.head) +: soFar.tail
  }
  
  def yesByAnyone(group: Seq[PersonAnswers]): Int = group.reduce(_ union _).size
  def yesByEveryone(group: Seq[PersonAnswers]): Int = group.reduce(_ intersect  _).size
  
  println(inputs.map(yesByAnyone).sum)
  println(inputs.map(yesByEveryone).sum)
}
