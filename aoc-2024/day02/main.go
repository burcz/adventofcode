package main

import (
	"bufio"
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)


func abs(a int64) int64 {
    if a >= 0 {
        return a
    }
    return -a
}


func readLines(path string) ([]string, error) {
    file, err := os.Open(path)
    if err != nil {
        return nil, err
    }
    defer file.Close()

    var lines []string
    scanner := bufio.NewScanner(file)
    for scanner.Scan() {
        lines = append(lines, scanner.Text())
    }
    return lines, scanner.Err()
}

func check(e error) {
    if e != nil {
        panic(e)
    }
}

func checkReport(report []int64) (int) {
  var dir int64
  var failures int
  dir = 0
  failures = 0
  for i, e := range report {
    if i > 0 {
      diff := e - report[i-1]
//      fmt.Println(dir)
      if abs(diff) > 3 || diff == 0 || dir != 0 && dir != (e - report[i-1])/abs(e - report[i-1]) {
//        fmt.Println(e, report[i-1])
//        fmt.Println("marked as unsafe")
//        fmt.Println(report)
        failures += 1
      }
      //fmt.Println(report, e, i)
      if e != report[i - 1] {
        dir = (e - report[i-1])/abs(e - report[i-1])
      }
    }
  }
// fmt.Println("marked as safe")
//  fmt.Println(report)
  return failures
}
func remove(slice []int64, s int) []int64 {
    return append(slice[:s], slice[s+1:]...)
}
func main() {

    lines, err := readLines("./input.txt")
    check(err)
    resa := 0
    resb := 0
    for lineNum := range lines {
      numstrings := strings.Split(lines[lineNum], " ")
      var nums []int64
      for _, n := range numstrings {
        converted, err := strconv.ParseInt(n, 10, 32)
        check(err)
        nums = append(nums, converted)

      }
      res := checkReport(nums)
      if res == 0 {
        resa += 1
        resb += 1
      } else {
        for i := range nums {
          new := remove(slices.Clone(nums), i)
          //fmt.Println(new)
          r := checkReport(new)
          if r == 0 {
            resb += 1
            break
          }
        }
      }

      check(err)

    }

    fmt.Printf("a: %d\n", resa)
    fmt.Printf("b: %d\n", resb)





}
