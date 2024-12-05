package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
  "slices"
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

func main() {

    lines, err := readLines("./input.txt")
    check(err)
    var l []int64
    var r []int64
    for lineNum := range lines {
      leftInt, err := strconv.ParseInt(strings.Split(lines[lineNum], "   ")[0], 10, 32)
      rightInt, err := strconv.ParseInt(strings.Split(lines[lineNum], "   ")[1], 10, 32)
      check(err)
      l = append(l, leftInt)
      r = append(r, rightInt)
    }
    slices.Sort(l)
    slices.Sort(r)

    var suma int64
    var sumb int64
    for i := range l {
      for j := range r {
        if l[i] == r[j] {
          sumb += l[i]
        }
      }
      suma += abs(r[i] - l[i])

    }

    fmt.Printf("a: %d\n", suma)
    fmt.Printf("b: %d\n", sumb)





}
