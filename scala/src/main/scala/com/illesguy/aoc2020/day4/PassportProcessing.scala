package com.illesguy.aoc2020.day4

import com.illesguy.aoc.common.Utils

object PassportProcessing extends App {
  val passportFieldsToCheck = Set("byr:", "iyr:", "eyr:", "hgt:", "hcl:", "ecl:", "pid:")
  val passportFieldsAndValuesToCheck = Set(
    raw".*byr:((19[2-9][0-9])|(200[0-2])).*".r,
    raw".*iyr:((201[0-9])|(2020)).*".r,
    raw".*eyr:((202[0-9])|(2030)).*".r,
    raw".*hgt:((((1[5-8][0-9])|(19[0-3]))cm)|(((59)|(6[0-9])|(7[0-6]))in)).*".r,
    raw".*hcl:#[a-f0-9]{6}.*".r,
    raw".*ecl:((amb)|(blu)|(brn)|(gry)|(grn)|(hzl)|(oth)).*".r,
    raw""".*pid:\d{9}((\D.*)|$$)""".r,
  )
  
  val inputs: Seq[String] = Utils.getInputForDay(2020, 4).foldLeft(Seq(Seq.empty[String])) {
    case (soFar, "") => Seq.empty[String] +: soFar
    case (soFar, inp) => (inp +: soFar.head) +: soFar.tail
  }.map(e => e.mkString(" "))
  
  def passportHasValidFields(pData: String) = passportFieldsToCheck.forall(pData.contains(_))
  def passportHasValidFieldsAndValues(pData: String): Boolean = {
    passportFieldsAndValuesToCheck.forall(r => pData.matches(r.regex))
  }

  val passportsWithValidFields = inputs.count(passportHasValidFields)
  println(passportsWithValidFields)
  
  val passportsWithValidFieldsAndValues = inputs.count(passportHasValidFieldsAndValues)
  println(passportsWithValidFieldsAndValues)
}
