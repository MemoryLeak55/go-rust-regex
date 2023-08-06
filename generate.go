//go:build rust
// +build rust

package main

import (
	"log"
	"os"
	"os/exec"
)

func main() {
	cmd := exec.Command("go", "generate")
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	err := cmd.Run()
	if err != nil {
		log.Fatal("could not build ", err)
	}
}
