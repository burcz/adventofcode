package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"regexp"
	"strconv"
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

func checkId(id int, ranges []string) bool {
	for _, rng := range ranges {
		boundaries := strings.Split(rng, "-")
		min, _ := strconv.Atoi(boundaries[0])
		max, _ := strconv.Atoi(boundaries[1])
		if id >= min && id <= max {
			return true
		}
	}
	return false
}

func runForInputA(filePath string) int {
	lines, err := readLines(filePath)
	check(err)
	ranges := []string{}
	ids := []int{}
	r, _ := regexp.Compile("-")
	for _, line := range lines {
		if line == "" {
			continue
		}
		if r.MatchString(line) {
			ranges = append(ranges, line)
		} else {
			id, _ := strconv.Atoi(line)
			ids = append(ids, id)
		}
	}
	res := 0
	for _, id := range ids {
		isFresh := checkId(id, ranges)
		if isFresh { res++ }
	}
	return res
}

func getMinMinAndMaxMax(ranges []string) [2]int {
	min := int(math.Inf(1))
	max := 0

	for _, rng := range ranges {
		boundaries := strings.Split(rng, "-")
		mn, _ := strconv.Atoi(boundaries[0])
		mx, _ := strconv.Atoi(boundaries[1])
		if mn < min { min = mn }
		if mx > max { max = mx }
	}
	return [2]int{min, max}
}

func runForInputB(filePath string) int {
	lines, err := readLines(filePath)
	check(err)
	ranges := []string{}
	for _, line := range lines {
		if line == "" {
			break
		}
		ranges = append(ranges, line)
	}
	boundaries := getMinMinAndMaxMax(ranges)
	// fmt.Printf("%d - %d", boundaries[0], boundaries[1])

	res := 0
	for i:= boundaries[0]; i <= boundaries[1]; i++ {
		isFresh := checkId(i, ranges)
		if isFresh { res++ }
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
