package main

import (
	"bufio"
	"fmt"
	"os"
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

func countNeighbours(grid [][]string, r int, c int) int {
	nb := 0
	for row := r - 1; row <= r + 1; row++ {
		if row < 0 || row >= len(grid) {
			continue
		}
		for col := c - 1; col <= c + 1; col++ {
			if col < 0 || col >= len(grid[0]) || r == row && c == col {
				continue
			}
			if grid[row][col] == "@" {
				nb++
			}
		}
	}
	return nb
}

func runForInputA(filePath string) int {
	lines, err := readLines(filePath)
	check(err)
	grid := [][]string{}
	for _, line := range lines {
		row := []string{}
		for _, elem := range line {
			row = append(row, string(elem))
			// print(string(elem))
		}
		grid = append(grid, row)
		// print("\n")
	}

	res := 0
	for r, row := range grid {
		for c, elem := range row {
			if (elem == "@" && countNeighbours(grid, r, c) < 4) {
				// fmt.Printf("%d %d", r, c)
				res++
			}
		}
	}

	return res
}

func runForInputB(filePath string) int {
	lines, err := readLines(filePath)
	check(err)
	grid := [][]string{}
	for _, line := range lines {
		row := []string{}
		for _, elem := range line {
			row = append(row, string(elem))
			// print(string(elem))
		}
		grid = append(grid, row)
		// print("\n")
	}

	res := 0
	needIteration := true
	for needIteration {
		removeCount := 0
		newGrid := grid
		for r, row := range grid {
			for c, elem := range row {
				if (elem == "@" && countNeighbours(grid, r, c) < 4) {
					// fmt.Printf("%d %d", r, c)
					removeCount++
					newGrid[r][c] = "."
				}
				// print(string(newGrid[r][c]))
			}
			// println()
		}
		res += removeCount
		grid = newGrid
		if removeCount == 0 {
			needIteration = false
		}
	}

	return res
}

func main() {

	var aTest = runForInputA("./input.test.txt")
	var a = runForInputA("./input.txt")
	var bTest = runForInputB("./input.test.txt")
	var b = runForInputB("./input.txt")

	fmt.Printf("a-test: %d\n", aTest)
	fmt.Printf("a: %d\n", a)
	fmt.Printf("b-test: %d\n", bTest)
	fmt.Printf("b: %d\n", b)
}
