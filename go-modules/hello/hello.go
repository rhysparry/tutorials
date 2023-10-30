package main

import (
	"fmt"
	"github.com/rhysparry/tutorials/go-modules/greetings"
)

func main() {
	message := greetings.Hello("Rhys")
	fmt.Println(message)
}
