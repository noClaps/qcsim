package qclang

import (
	"fmt"
	"slices"
	"strconv"

	"github.com/noclaps/qcsim/computer"
	"github.com/noclaps/qcsim/qubit"
	tree_sitter_qc "github.com/noclaps/qcsim/tree-sitter-qc/bindings/go"
	tree_sitter "github.com/tree-sitter/go-tree-sitter"
)

type gate uint

const (
	measure       gate = iota // measure(uint?)
	pauliX                    // x(qubit)
	pauliY                    // y(qubit)
	pauliZ                    // z(qubit)
	hadamard                  // hadamard(qubit)
	phase                     // phase(qubit)
	piBy8                     // pi_8(qubit)
	controlledNot             // cnot(qubit, qubit)
	controlledZ               // cz(qubit, qubit)
	swap                      // swap(qubit, qubit)
	toffoli                   // toffoli(qubit, qubit, qubit)
)

type qcVariable struct {
	name  string
	qubit qubit.Qubit
}
type qcFunction struct {
	instruction gate
	arguments   []string
}

type qcLang struct {
	input        string
	variables    []qcVariable
	instructions []qcFunction
}

func New(input string) qcLang {
	return qcLang{input, []qcVariable{}, []qcFunction{}}
}

func (q *qcLang) Parse() error {
	parser := tree_sitter.NewParser()
	defer parser.Close()
	parser.SetLanguage(tree_sitter.NewLanguage(tree_sitter_qc.Language()))

	tree := parser.Parse([]byte(q.input), nil)
	defer tree.Close()

	nodes := []tree_sitter.Node{*tree.RootNode()}
	return q.parseTree(nodes, 0)
}

func (q *qcLang) Run() error {
	variables := q.variables

	qubits := []qubit.Qubit{}
	for _, variable := range variables {
		qubits = append(qubits, variable.qubit)
	}
	computer := computer.New(qubits)

	for _, instruction := range q.instructions {
		instr := instruction.instruction
		args := instruction.arguments

		switch instr {
		case measure:
			var count uint = 1
			if len(args) == 1 {
				if countArg, err := strconv.ParseUint(args[0], 10, 0); err != nil {
					return err
				} else {
					count = uint(countArg)
				}
			}
			if count == 1 {
				fmt.Println(computer.Measure())
				continue
			}

			outputs := make(map[string]uint)
			for range count {
				measuredState := computer.Measure()
				if val, ok := outputs[measuredState]; ok {
					outputs[measuredState] = val + 1
				} else {
					outputs[measuredState] = 1
				}
			}
			fmt.Println(formatOutputs(outputs))
		case pauliX:
			arg := args[0]
			index := slices.IndexFunc(variables, func(variable qcVariable) bool {
				return variable.name == arg
			})
			if index == -1 {
				return fmt.Errorf("Variable used but not declared: %s", arg)
			}
			if err := computer.PauliX(uint(index)); err != nil {
				return err
			}
		case pauliY:
			arg := args[0]
			index := slices.IndexFunc(variables, func(variable qcVariable) bool {
				return variable.name == arg
			})
			if index == -1 {
				return fmt.Errorf("Variable used but not declared: %s", arg)
			}
			if err := computer.PauliY(uint(index)); err != nil {
				return err
			}
		case pauliZ:
			arg := args[0]
			index := slices.IndexFunc(variables, func(variable qcVariable) bool {
				return variable.name == arg
			})
			if index == -1 {
				return fmt.Errorf("Variable used but not declared: %s", arg)
			}
			if err := computer.PauliZ(uint(index)); err != nil {
				return err
			}
		case hadamard:
			arg := args[0]
			index := slices.IndexFunc(variables, func(variable qcVariable) bool {
				return variable.name == arg
			})
			if index == -1 {
				return fmt.Errorf("Variable used but not declared: %s", arg)
			}
			if err := computer.Hadamard(uint(index)); err != nil {
				return err
			}
		case phase:
			arg := args[0]
			index := slices.IndexFunc(variables, func(variable qcVariable) bool {
				return variable.name == arg
			})
			if index == -1 {
				return fmt.Errorf("Variable used but not declared: %s", arg)
			}
			if err := computer.Phase(uint(index)); err != nil {
				return err
			}
		case piBy8:
			arg := args[0]
			index := slices.IndexFunc(variables, func(variable qcVariable) bool {
				return variable.name == arg
			})
			if index == -1 {
				return fmt.Errorf("Variable used but not declared: %s", arg)
			}
			if err := computer.PiBy8(uint(index)); err != nil {
				return err
			}
		case controlledNot:
			indexes := []uint{}
			for i := range 2 {
				arg := args[i]
				index := slices.IndexFunc(variables, func(variable qcVariable) bool {
					return variable.name == arg
				})
				if index == -1 {
					return fmt.Errorf("Variable used but not declared: %s", arg)
				}
				indexes = append(indexes, uint(index))
			}

			if err := computer.ControlledNot(indexes[0], indexes[1]); err != nil {
				return err
			}
		case controlledZ:
			indexes := []uint{}
			for i := range 2 {
				arg := args[i]
				index := slices.IndexFunc(variables, func(variable qcVariable) bool {
					return variable.name == arg
				})
				if index == -1 {
					return fmt.Errorf("Variable used but not declared: %s", arg)
				}
				indexes = append(indexes, uint(index))
			}

			if err := computer.ControlledZ(indexes[0], indexes[1]); err != nil {
				return err
			}
		case swap:
			indexes := []uint{}
			for i := range 2 {
				arg := args[i]
				index := slices.IndexFunc(variables, func(variable qcVariable) bool {
					return variable.name == arg
				})
				if index == -1 {
					return fmt.Errorf("Variable used but not declared: %s", arg)
				}
				indexes = append(indexes, uint(index))
			}

			if err := computer.Swap(indexes[0], indexes[1]); err != nil {
				return err
			}
		case toffoli:
			indexes := []uint{}
			for i := range 3 {
				arg := args[i]
				index := slices.IndexFunc(variables, func(variable qcVariable) bool {
					return variable.name == arg
				})
				if index == -1 {
					return fmt.Errorf("Variable used but not declared: %s", arg)
				}
				indexes = append(indexes, uint(index))
			}
			if err := computer.Toffoli(indexes[0], indexes[1], indexes[2]); err != nil {
				return err
			}
		}
	}

	return nil
}
