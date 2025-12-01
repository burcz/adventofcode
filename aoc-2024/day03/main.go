package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"slices"
	"strconv"
)

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

func calc(input string) (int) {
    r, _ := regexp.Compile("mul\\((\\d{1,3}),(\\d{1,3})\\)")
    nums := r.FindStringSubmatch(input)
    //fmt.Printf("%q\n", nums)
    x, _ := strconv.Atoi(nums[1])
    y, _ := strconv.Atoi(nums[2])

    return x * y
}

func calcRanges(len int, d [][]int, n [][]int) ([]int) {
  write := true
  var dstarts []int
  for _, e:= range d {
    dstarts = append(dstarts, e[0])
  }
  var nstarts []int
  for _, e:= range n {
    nstarts = append(nstarts, e[0])
  }
  fmt.Println(dstarts)
  fmt.Println(nstarts)
  var res []int
  for i := range len {
    if slices.Contains(dstarts, i) {
      write = true
    }
    if slices.Contains(nstarts, i) {
      write = false
    }
    if write {
      res = append(res, i)
    }
  }
  return res
}


func main() {
    var resa int
    var resb int
    resa = 0
    resb = 0
    lines, err := readLines("./input.txt")
    check(err)
    r, _ := regexp.Compile("mul\\(\\d{1,3},\\d{1,3}\\)")
    d, _ := regexp.Compile("do\\(\\)")
    n, e := regexp.Compile("don't\\(\\)")
    check(e)
    for _, line := range lines {
      findexes := r.FindAllIndex([]byte(line), -1)
      dindexes := d.FindAllIndex([]byte(line), -1)
      nindexes := n.FindAllIndex([]byte(line), -1)
      fmt.Println("nind", nindexes)
      ranges := calcRanges(len(line), dindexes, nindexes)
      fmt.Println(line)
      found := r.FindAll([]byte(line), -1)
      fmt.Println(ranges)

      for i, e := range found {
        //fmt.Printf("%q\n", e)
        resa += calc(string(e[:]))
        if slices.Contains(ranges, findexes[i][0]) {
          fmt.Println(string(e[:]))
          resb += calc(string(e[:]))
        }
      }
    }
    fmt.Printf("a: %d\n", resa)
    fmt.Printf("b: %d\n", resb)
}
