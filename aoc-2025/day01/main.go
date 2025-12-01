package main

import (
	"bufio"
	"fmt"
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

func fixNewValA(newVal int) int {
	MAX := 99
	fixed := false

	for !fixed {
		if newVal < 0 {
			newVal = MAX + newVal + 1
		}

		if newVal > MAX {
			newVal = newVal - MAX - 1
		}
		if newVal >= 0 && newVal <= MAX {
			fixed = true
		}
	}
	return newVal
}

func fixNewValB(cur int, value int, newVal int, tick int) (int, int) {
	MAX := 99

	if newVal < 0 {
		if cur != 0 {
			//println("inc1")
			tick++
		}
		newVal = MAX + newVal + 1
	}

	if newVal > MAX {
		newVal = newVal - MAX - 1
		if newVal != 0 {
			//println("inc1")
			tick++
		}
	}

	if (newVal < 0 || newVal > MAX) {
		newVal, tick  = fixNewValB(1, value, newVal, tick)
	}

	return newVal, tick
}

func rotateA(cur int, dir string, value int) int {
	var newVal = cur
	if dir == "L" {
		newVal = cur - value
	} else {
		newVal = cur + value
	}

	return fixNewValA(newVal)
}

func rotateB(cur int, dir string, value int, tick int) (int, int) {
	var newVal int
	if dir == "L" {
		newVal = cur - value
	} else {
		newVal = cur + value
	}

	newVal, tick = fixNewValB(cur, value, newVal, tick)

  if newVal == 0 {
		tick++
		//println("inc0")
	}
	//fmt.Printf("%d %s%d %d %d\n", cur, dir, value, newVal, tick)
	return newVal, tick
}

type Command struct {
	dir string
	value int
}

func runForInputA(fileName string) int {
    lines, err := readLines(fileName)
    check(err)

    var commands []Command

    for lineNum := range lines {
			dir := string(lines[lineNum][0])
			numPart := lines[lineNum][1:]
			value, err := strconv.Atoi(numPart)

			check(err)

			var command = Command{
				dir,
				value,
			}

			commands = append(commands, command)
    }

		var cur = 50
		var numZeros = 0

		for _, command := range commands {
			cur = rotateA(cur, command.dir, command.value)
			if cur == 0 {
				numZeros++
			}
		}
		return numZeros
}

func runForInputB(fileName string) int {
    lines, err := readLines(fileName)
    check(err)

    var commands []Command

    for lineNum := range lines {
			dir := string(lines[lineNum][0])
			numPart := lines[lineNum][1:]
			value, err := strconv.Atoi(numPart)

			check(err)

			var command = Command{
				dir,
				value,
			}

			commands = append(commands, command)
    }

		cur := 50
		tick := 0

		for _, command := range commands {
			cur, tick = rotateB(cur, command.dir, command.value, tick)
			//if i == 280 { break }
		}
		return tick
}

func main() {

	var aTest = runForInputA("./input.test.txt")
	var a = runForInputA("./input.txt")

	var bTest = runForInputB("./input.test.txt")
	var b = runForInputB("./input.txt")

		  fmt.Printf("a: %d\n", a)
		fmt.Printf("a-test: %d\n", aTest)
		  fmt.Printf("b-test: %d\n", bTest)
		fmt.Printf("b: %d\n", b)
    // fmt.Printf("b: %d\n", sumb)
}
