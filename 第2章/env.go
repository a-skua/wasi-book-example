package main

import (
	"fmt"
	"os"
)

func main() {
	// 環境変数
	env := os.Getenv("EXAMPLE")
	fmt.Println("EXAMPLE =", env)

	// 引数
	args := os.Args
	fmt.Println("ARGS =", args)
}
