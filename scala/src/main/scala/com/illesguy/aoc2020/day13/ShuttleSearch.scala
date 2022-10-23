package com.illesguy.aoc2020.day13

import com.illesguy.aoc.common.Utils

object ShuttleSearch extends App {
  case class CommonTime(difference: BigInt, busNumber: BigInt, arrivalTime: BigInt)
  
  val input = Utils.getInputForDay(2020, 13)
  val times = input(1).split(",").zipWithIndex.flatMap {
    case ("x", _) =>
      None
    case (busNumber, index) =>
      Some(CommonTime(index.toInt, busNumber.toInt, busNumber.toInt))
  }

  def gcd(a: BigInt, b: BigInt): BigInt = if (b == 0) a.abs else gcd(b, a % b)
  def lcm(a: BigInt, b: BigInt): BigInt = (a / gcd(a, b)) * b
  
  def mergeCommonTimes(left: CommonTime, right: CommonTime): CommonTime = {
    val diffToUse = right.difference % right.busNumber
    def iter(leftTime: BigInt): BigInt = {
      if (leftTime % right.arrivalTime != right.arrivalTime - diffToUse)
        iter(leftTime + left.busNumber)
      else
        leftTime
    }
    
    CommonTime(0L, lcm(left.busNumber, right.busNumber), iter(left.arrivalTime))
  }
  
  println(times.reduce(mergeCommonTimes).arrivalTime)
}
