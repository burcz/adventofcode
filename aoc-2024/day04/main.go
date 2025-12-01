package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
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

  type coord struct {
    x int
    y int
  }
func findChar(grid [][]string, character string) ([]coord){
  var res []coord
  for r, row := range grid {
    for c, char := range row {
      if char == character {
        var cr coord
        cr.x = c
        cr.y = r
        res = append(res, cr)
      }
    }
  }
  return res
}

func checkHorRight(input [][]string, c coord) (bool) {
  if len(input[c.y]) > c.x + 3 {
    if input[c.y][c.x + 1] == "M" && input[c.y][c.x + 2] == "A" && input[c.y][c.x + 3] == "S" {
      return true
    }
  }
  return false
}
func checkHorLeft(input [][]string, c coord) (bool) {
  if 0 <= c.x - 3 {
    if input[c.y][c.x - 1] == "M" && input[c.y][c.x - 2] == "A" && input[c.y][c.x - 3] == "S" {
      return true
    }
  }
  return false
}

func checkVerBot(input [][]string, c coord) (bool) {
  if len(input) > c.y + 3 {
    if input[c.y + 1][c.x] == "M" && input[c.y + 2][c.x] == "A" && input[c.y + 3][c.x] == "S" {
      return true
    }
  }
  return false
}

func checkVerTop(input [][]string, c coord) (bool) {
  if 0 <= c.y - 3 {
    if input[c.y - 1][c.x] == "M" && input[c.y - 2][c.x] == "A" && input[c.y - 3][c.x] == "S" {
      return true
    }
  }
  return false
}

func checkRB(input [][]string, c coord) (bool) {
  if len(input) > c.y + 3 && len(input[c.y]) > c.x + 3 {
    if input[c.y + 1][c.x + 1] == "M" && input[c.y + 2][c.x + 2] == "A" && input[c.y + 3][c.x + 3] == "S" {
      return true
    }
  }
  return false
}
func checkRT(input [][]string, c coord) (bool) {
  if 0 <= c.y - 3 && len(input[c.y]) > c.x + 3 {
    if input[c.y - 1][c.x + 1] == "M" && input[c.y - 2][c.x + 2] == "A" && input[c.y - 3][c.x + 3] == "S" {
      return true
    }
  }
  return false
}
func checkLT(input [][]string, c coord) (bool) {
  if 0 <= c.y - 3 && 0 <= c.x - 3  {
    if input[c.y - 1][c.x - 1] == "M" && input[c.y - 2][c.x - 2] == "A" && input[c.y - 3][c.x - 3] == "S" {
      return true
    }
  }
  return false
}
func checkLB(input [][]string, c coord) (bool) {
  if len(input) > c.y + 3 && 0 <= c.x - 3  {
    if input[c.y + 1][c.x - 1] == "M" && input[c.y + 2][c.x - 2] == "A" && input[c.y + 3][c.x - 3] == "S" {
      return true
    }
  }
  return false
}
func findCross(input [][]string, c coord) (bool) {
  a := false
  b := false
  if len(input) > c.y + 1 && 0 <= c.x - 1 && len(input) > c.x + 1 && 0 <= c.y - 1  {
    if input[c.y + 1][c.x - 1] == "M" && input[c.y - 1][c.x + 1] == "S" {
      a = true
    }
    if input[c.y + 1][c.x - 1] == "S" && input[c.y - 1][c.x + 1] == "M" {
      a = true
    }
    if input[c.y - 1][c.x - 1] == "M" && input[c.y + 1][c.x + 1] == "S" {
      b = true
    }
    if input[c.y - 1][c.x - 1] == "S" && input[c.y + 1][c.x + 1] == "M" {
      b = true
    }
  }
  return a && b
}
func main() {
    var resa int
    var resb int
    lines, err := readLines("./input.txt")
    check(err)
    input := make([][]string, len(lines))
    for r, line := range lines {
      input[r] = make([]string, len(strings.Split(lines[0], "")))
      for c, character := range strings.Split(line, "") {
        input[r][c] = character
      }
    }
    for row := 0; row < len(input); row++ {
      for column := 0; column < len(input[row]); column++{
        fmt.Print(input[row][column], "")
      }
      fmt.Print("\n")
    }
    xs := findChar(input, "X")
    for _, c := range xs {
      if checkHorRight(input, c) {
        resa++
      }
      if checkHorLeft(input, c) {
        resa++
      }
      if checkVerBot(input, c) {
        resa++
      }
      if checkVerTop(input, c) {
        resa++
      }
      if checkRB(input, c) {
        resa++
      }
      if checkRT(input, c) {
        resa++
      }
      if checkLB(input, c) {
        resa++
      }
      if checkLT(input, c) {
        resa++
      }
    }
    as := findChar(input, "A")
    for _, c := range as {
      fmt.Println(c)
      if findCross(input, c) {
        resb++
      }
    }



    fmt.Printf("a: %d\n", resa)
    fmt.Printf("b: %d\n", resb)
}
