package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strings"
)

func main() {
	path := os.Args[1]

	data, _ := ioutil.ReadFile(path)
	input := strings.Split(string(data), "\n\n")

	result := 0
	for i := range input {
		group := strings.Split(string(input[i]), "\n")
		groupCount := len(group)
		questions := make(map[string]int)
		for k := range group {
			for _, char := range group[k] {
				questions[string(char)]++

				if questions[string(char)] == groupCount {
					result++
				}
			}
		}
	}

	fmt.Println(result)
}
