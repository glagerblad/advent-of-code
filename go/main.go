package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func input() []string {
	rdata, err := os.Open("../inputs/day_5.txt")
	if err != nil {
		panic(err)
	}
	defer rdata.Close()

	if err != nil {
		fmt.Println(err)
	}
	scanner := bufio.NewScanner(rdata)

	scanner.Split(bufio.ScanLines)

	input := []string{}
	for scanner.Scan() {
		input = append(input, scanner.Text())
	}
	return input
}

// TBVFVDZPN
func part_1() {
	input := input()[10:]
	stacks := [][]string{
		[]string{"V", "C", "D", "R", "Z", "G", "B", "W"},
		[]string{"G", "W", "F", "C", "B", "S", "T", "V"},
		[]string{"C", "B", "S", "N", "W"},
		[]string{"Q", "G", "M", "N", "J", "V", "C", "P"},
		[]string{"T", "S", "L", "F", "D", "H", "B"},
		[]string{"J", "V", "T", "W", "M", "N"},
		[]string{"P", "F", "L", "C", "S", "T", "G"},
		[]string{"B", "D", "Z"},
		[]string{"M", "N", "Z", "W"},
	}
	for _, line := range input {
		instr := strings.Split(line, " ")
		amount, _ := strconv.Atoi(instr[1])
		maybe_from, _ := strconv.Atoi(instr[3])
		maybe_to, _ := strconv.Atoi(instr[5])
		from := maybe_from - 1
		to := maybe_to - 1
		taken := stacks[from][len(stacks[from])-amount:]
		//reverse slice
		for i, j := 0, len(taken)-1; i < j; i, j = i+1, j-1 {
			taken[i], taken[j] = taken[j], taken[i]
		}
		stacks[to] = append(stacks[to], taken...)
		stacks[from] = stacks[from][:len(stacks[from])-amount]
	}
	answer := ""
	for _, stack := range stacks {
		answer += stack[len(stack)-1:][0]
	}
	fmt.Println(answer)
}

// VLCWHTDSZ
func part_2() {
	input := input()[10:]
	stacks := [][]string{
		[]string{"V", "C", "D", "R", "Z", "G", "B", "W"},
		[]string{"G", "W", "F", "C", "B", "S", "T", "V"},
		[]string{"C", "B", "S", "N", "W"},
		[]string{"Q", "G", "M", "N", "J", "V", "C", "P"},
		[]string{"T", "S", "L", "F", "D", "H", "B"},
		[]string{"J", "V", "T", "W", "M", "N"},
		[]string{"P", "F", "L", "C", "S", "T", "G"},
		[]string{"B", "D", "Z"},
		[]string{"M", "N", "Z", "W"},
	}
	for _, line := range input {
		instr := strings.Split(line, " ")
		amount, _ := strconv.Atoi(instr[1])
		maybe_from, _ := strconv.Atoi(instr[3])
		maybe_to, _ := strconv.Atoi(instr[5])
		from := maybe_from - 1
		to := maybe_to - 1
		taken := stacks[from][len(stacks[from])-amount:]
		stacks[to] = append(stacks[to], taken...)
		stacks[from] = stacks[from][:len(stacks[from])-amount]
	}
	answer := ""
	for _, stack := range stacks {
		answer += stack[len(stack)-1:][0]
	}
	fmt.Println(answer)
}

func main() {
	part_1()
	part_2()
}
