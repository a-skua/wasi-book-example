package main

import (
	"fmt"
)

func main() {
	fmt.Print("Your name: ")

	// 標準入力
	var name string
	fmt.Scan(&name)

	// 標準出力
	fmt.Printf("Hello, %s!\n", name)
}
