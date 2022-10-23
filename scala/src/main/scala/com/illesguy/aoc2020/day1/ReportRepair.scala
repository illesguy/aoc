package com.illesguy.aoc2020.day1

import com.illesguy.aoc.common.Utils

object ReportRepair extends App {
  
  def validateReports(reports: Int*): Option[Int] =
    if (reports.sum == 2020)
      Some(reports.reduce(_ * _))
    else
      None
  
  val reports = Utils.getInputForDay(2020, 1).map(_.toInt)
  val validReportPairs = (for {
    i <- (0 until reports.size - 2)
    j <- (i until reports.size - 1)
  } yield validateReports(reports(i), reports(j))).flatten
  
  validReportPairs.headOption.foreach(println)

  val validReportTrios = (for {
    i <- (0 until reports.size - 3)
    j <- (i until reports.size - 2)
    k <- (j until reports.size - 1)
  } yield validateReports(reports(i), reports(j), reports(k))).flatten

  validReportTrios.headOption.foreach(println)
}
