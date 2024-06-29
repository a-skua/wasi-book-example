package main

import (
	"fmt"
	"io"
	"os"
)

func main() {
	path := os.Args[1]
	file, err := os.Open(path)
	if err != nil {
		panic(err)
	}
	defer file.Close()

	contents, err := io.ReadAll(file)
	if err != nil {
		panic(err)
	}

	fmt.Println(string(contents))
}
