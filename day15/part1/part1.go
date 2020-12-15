package main

import (
	"fmt"
)

func main() {
	input := [7]int{11, 18, 0, 20, 1, 7, 16}
	target := 2020
	spoken := make(map[int][]int)

	for index, val := range input {
		spoken[val] = append(spoken[val], index)
	}

	lastSpoken := input[len(input)-1]
	for i := len(input); i < target; i++ {
		var nextNumber int

		if res, ok := spoken[lastSpoken]; ok && len(res) > 1 {
			lastOccurence := res[len(res)-1]
			oneBeforeLast := res[len(res)-2]
			nextNumber = lastOccurence - oneBeforeLast
		} else {
			nextNumber = 0
		}

		lastSpoken = nextNumber
		spoken[lastSpoken] = append(spoken[lastSpoken], i)
	}

	fmt.Println(lastSpoken)
}
