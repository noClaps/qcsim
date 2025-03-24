package main

import (
	"flag"
	"fmt"
	"log"
	"os"
	"slices"
	"strings"

	"github.com/noclaps/qcsim/qclang"
)

func findArg(argv []string) (string, []string) {
	for i := range argv {
		if (i == 0 && argv[i][0] != '-') || (i-1 >= 0 && argv[i-1][0] != '-') {
			return argv[i], slices.Concat(argv[:i], argv[i+1:])
		}
	}

	return "", argv
}

func main() {
	var help bool
	flag.BoolVar(&help, "help", false, "Display this help message and exit")
	flag.BoolVar(&help, "h", false, "Display this help message and exit")

	inputFile, remainingArgs := findArg(os.Args[1:])
	err := flag.CommandLine.Parse(remainingArgs)

	if help || err != nil {
		fmt.Println(strings.TrimSpace(`
USAGE: qcsim <input>

ARGUMENTS:
  <input>     QC Instructions file to be run

OPTIONS:
  -h, --help  Display this help message and exit
`))
		return
	}

	file, err := os.ReadFile(inputFile)
	if err != nil {
		log.Fatalln("[ERROR]", err)
	}

	qc := qclang.New(string(file))
	if err = qc.Parse(); err != nil {
		log.Fatalln("[ERROR]", err)
	}
	if err = qc.Run(); err != nil {
		log.Fatalln("[ERROR]", err)
	}
}
