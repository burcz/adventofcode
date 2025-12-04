package main

import (
	"bufio"
	"fmt"
	"os"
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

func getRanges(input string) [][]int {
	ranges := strings.Split(input, ",")
	var result [][]int
	for _, interval := range ranges {
		e := strings.Split(interval, "-")
		start, e1 := strconv.Atoi(e[0])
		end, e2 := strconv.Atoi(e[1])
		check(e1)
		check(e2)
		result = append(result, []int{start, end})
	}
	return result
}

func runForInputA(filePath string) int {
	lines, err := readLines(filePath)
	check(err)
	var invalids []int
	res := getRanges(lines[0])
	for _, rng := range res {
		for elem := rng[0]; elem <= rng[1]; elem++ {
			stringed := strconv.Itoa(elem)
			length := len(stringed)
			if length % 2 == 0 {
				if stringed[:length / 2] == stringed[length / 2:] {
					invalids = append(invalids, elem)
				}
			}
		}
	}
	sum := 0
	for _, num := range invalids {
		sum += num
	}
	return sum
}

func testPattern(pattern int, num int, original int) (int, bool) {
	strPattern := strconv.Itoa(pattern)
	strNum := strconv.Itoa(num)
	strOriginal := strconv.Itoa(original)

	if len(strNum) % len(strPattern) != 0 || len(strPattern) == len(strOriginal) {
		return 0, false
	}
	// fmt.Printf("%d %d %d\n", pattern, num, original)

	if strNum[:len(strPattern)] == strPattern {
		if len(strPattern) == len(strNum) {
			return original, true
		}

		slice := strNum[len(strPattern):]
		if string(slice[0]) == "0" {
			return 0, false
		}
		newNum, err := strconv.Atoi(slice)
		check(err)
		return testPattern(pattern, newNum, original)
	}


	return 0, false
}

func runForInputB(filePath string) int {
	lines, err := readLines(filePath)
	check(err)
	var invalids []int
	res := getRanges(lines[0])
	for _, rng := range res {
		for elem := rng[0]; elem <= rng[1]; elem++ {
			stringed := strconv.Itoa(elem)
			for i := 1; i <= len(stringed)/2; i++ {
				pattern, err := strconv.Atoi(stringed[:i])
				check(err)
				match, found := testPattern(pattern, elem, elem)
				if found {
					invalids = append(invalids, match)
					break
				}
			}
		}
	}
	sum := 0
	for _, num := range invalids {
		// println(num)
		sum += num
	}
	return sum
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
