//go:build rust
// +build rust

package main

import (
	"log"
	"os"
	"os/exec"
)

func main() {
	cmd := exec.Command("mkdir", "-p", "lib/rust-re/target/release")
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	err := cmd.Run()
	if err != nil {
		log.Fatal("could not build ", err)
	}

	cmd = exec.Command("rm", "-rf", "/tmp/go-rust-regex")
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	err = cmd.Run()
	if err != nil {
		log.Fatal("could not build ", err)
	}

	cmd = exec.Command("git", "clone", "git@github.com:thomas-thorburn-connect/go-rust-regex.git", "/tmp/go-rust-regex")
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	err = cmd.Run()
	if err != nil {
		log.Fatal("could not build ", err)
	}

	cmd = exec.Command("cargo", "clean", "--manifest-path=/tmp/go-rust-regex/lib/rust-re/Cargo.toml")
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	err = cmd.Run()
	if err != nil {
		log.Fatal("could not build ", err)
	}

	cmd = exec.Command("cargo", "build", "--release", "--manifest-path=/tmp/go-rust-regex/lib/rust-re/Cargo.toml")
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	err = cmd.Run()
	if err != nil {
		log.Fatal("could not build ", err)
	}

	cmd = exec.Command("cp", "/tmp/go-rust-regex/lib/rust-re/target/release/librust_re.a", "lib/rust-re/target/release/librust_re.a")
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	err = cmd.Run()
	if err != nil {
		log.Fatal("could not build ", err)
	}

	cmd = exec.Command("cp", "/tmp/go-rust-regex/lib/rust-re.h", "lib/rust-re/rust_re.h")
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	err = cmd.Run()
	if err != nil {
		log.Fatal("could not build ", err)
	}

	cmd = exec.Command("rm", "-r", "/tmp/go-rust-regex")
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	err = cmd.Run()
	if err != nil {
		log.Fatal("could not build ", err)
	}

}
