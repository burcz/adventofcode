package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
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

func calculateHighestJoltageA(batteries string) int {
	first := [2]int{ 0, 0 }
	second := [2]int{ 0, 0 }
	for i, val := range batteries {
		if i + 1 == len(batteries) {
			continue
		}
		valStr, e := strconv.Atoi(string(val))
		check(e)
		if valStr > first[1] {
			first[0] = i
			first[1] = valStr
		}
	}
	for i := first[0]+1; i < len(batteries); i++ {
		valStr, e := strconv.Atoi(string(batteries[i]))
		check(e)
		if valStr > second[1] {
			second[0] = i
			second[1] = valStr
		}
	}
	return first[1]*10 + second[1]
}

func calculateHighestJoltageWithThreshold(batteries string, start int, threshold int) [2]int {
	max := [2]int{ 0, 0 }
	for i := start; i < len(batteries) - threshold; i++ {
		valStr, e := strconv.Atoi(string(batteries[i]))
		check(e)
		if valStr > max[1] {
			max[0] = i
			max[1] = valStr
		}
	}
	return max
}

func calculateHighestJoltageB(batteries string) int {
	digit := calculateHighestJoltageWithThreshold(batteries, 0, 11)
	result := int(math.Pow10(11)) * digit[1]
	for d := 2; d <= 12; d++ {
		// println(digit[0], digit[1])
		digit = calculateHighestJoltageWithThreshold(batteries, digit[0]+1, 12-d)
		result += int(math.Pow10(12-d)) * digit[1]
	}

	return result
}

func runForInputA(path string) int {
	lines, err := readLines(path)
	check(err)
	sum := 0
	for _, line := range lines {
		joltage := calculateHighestJoltageA(line)
		sum += joltage
	}
	return sum
}

func runForInputB(path string) int {
	lines, err := readLines(path)
	check(err)
	sum := 0
	for _, line := range lines {
		joltage := calculateHighestJoltageB(line)
		// println(joltage)
		sum += joltage
	}
	return sum
}

func main() {

	var aTest = runForInputA("./input.test.txt")
	var a = runForInputA("./input.txt")
	var bTest = runForInputB("./input.test.txt")
	var b = runForInputB("./input.txt")
	//
	fmt.Printf("a-test: %d\n", aTest)
	fmt.Printf("a: %d\n", a)
	fmt.Printf("b-test: %d\n", bTest)
	fmt.Printf("b: %d\n", b)
}
