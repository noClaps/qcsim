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

type Instruction uint

const (
	Measure       Instruction = iota // measure(uint?)
	PauliX                           // x(qubit)
	PauliY                           // y(qubit)
	PauliZ                           // z(qubit)
	Hadamard                         // hadamard(qubit)
	Phase                            // phase(qubit)
	PiBy8                            // pi_8(qubit)
	ControlledNot                    // cnot(qubit, qubit)
	ControlledZ                      // cz(qubit, qubit)
	Swap                             // swap(qubit, qubit)
	Toffoli                          // toffoli(qubit, qubit, qubit)
)

type QCVariable struct {
	Name  string
	Qubit qubit.Qubit
}
type QCFunction struct {
	Instruction Instruction
	Arguments   []string
}

type QCLang struct {
	Input        string
	Variables    []QCVariable
	Instructions []QCFunction
}

func New(input string) QCLang {
	return QCLang{input, []QCVariable{}, []QCFunction{}}
}

func (q *QCLang) Parse() error {
	parser := tree_sitter.NewParser()
	defer parser.Close()
	parser.SetLanguage(tree_sitter.NewLanguage(tree_sitter_qc.Language()))

	tree := parser.Parse([]byte(q.Input), nil)
	defer tree.Close()

	nodes := []tree_sitter.Node{*tree.RootNode()}
	return q.parseTree(nodes, 0)
}

func (q *QCLang) Run() error {
	variables := q.Variables

	qubits := []qubit.Qubit{}
	for _, variable := range variables {
		qubits = append(qubits, variable.Qubit)
	}
	computer := computer.New(qubits)

	for _, instruction := range q.Instructions {
		instr := instruction.Instruction
		args := instruction.Arguments

		switch instr {
		case Measure:
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
		case PauliX:
			arg := args[0]
			index := slices.IndexFunc(variables, func(variable QCVariable) bool {
				return variable.Name == arg
			})
			if index == -1 {
				return fmt.Errorf("Variable used but not declared: %s", arg)
			}
			if err := computer.PauliX(uint(index)); err != nil {
				return err
			}
		case PauliY:
			arg := args[0]
			index := slices.IndexFunc(variables, func(variable QCVariable) bool {
				return variable.Name == arg
			})
			if index == -1 {
				return fmt.Errorf("Variable used but not declared: %s", arg)
			}
			if err := computer.PauliY(uint(index)); err != nil {
				return err
			}
		case PauliZ:
			arg := args[0]
			index := slices.IndexFunc(variables, func(variable QCVariable) bool {
				return variable.Name == arg
			})
			if index == -1 {
				return fmt.Errorf("Variable used but not declared: %s", arg)
			}
			if err := computer.PauliZ(uint(index)); err != nil {
				return err
			}
		case Hadamard:
			arg := args[0]
			index := slices.IndexFunc(variables, func(variable QCVariable) bool {
				return variable.Name == arg
			})
			if index == -1 {
				return fmt.Errorf("Variable used but not declared: %s", arg)
			}
			if err := computer.Hadamard(uint(index)); err != nil {
				return err
			}
		case Phase:
			arg := args[0]
			index := slices.IndexFunc(variables, func(variable QCVariable) bool {
				return variable.Name == arg
			})
			if index == -1 {
				return fmt.Errorf("Variable used but not declared: %s", arg)
			}
			if err := computer.Phase(uint(index)); err != nil {
				return err
			}
		case PiBy8:
			arg := args[0]
			index := slices.IndexFunc(variables, func(variable QCVariable) bool {
				return variable.Name == arg
			})
			if index == -1 {
				return fmt.Errorf("Variable used but not declared: %s", arg)
			}
			if err := computer.PiBy8(uint(index)); err != nil {
				return err
			}
		case ControlledNot:
			indexes := []uint{}
			for i := range 2 {
				arg := args[i]
				index := slices.IndexFunc(variables, func(variable QCVariable) bool {
					return variable.Name == arg
				})
				if index == -1 {
					return fmt.Errorf("Variable used but not declared: %s", arg)
				}
				indexes = append(indexes, uint(index))
			}

			if err := computer.ControlledNot(indexes[0], indexes[1]); err != nil {
				return err
			}
		case ControlledZ:
			indexes := []uint{}
			for i := range 2 {
				arg := args[i]
				index := slices.IndexFunc(variables, func(variable QCVariable) bool {
					return variable.Name == arg
				})
				if index == -1 {
					return fmt.Errorf("Variable used but not declared: %s", arg)
				}
				indexes = append(indexes, uint(index))
			}

			if err := computer.ControlledZ(indexes[0], indexes[1]); err != nil {
				return err
			}
		case Swap:
			indexes := []uint{}
			for i := range 2 {
				arg := args[i]
				index := slices.IndexFunc(variables, func(variable QCVariable) bool {
					return variable.Name == arg
				})
				if index == -1 {
					return fmt.Errorf("Variable used but not declared: %s", arg)
				}
				indexes = append(indexes, uint(index))
			}

			if err := computer.Swap(indexes[0], indexes[1]); err != nil {
				return err
			}
		case Toffoli:
			indexes := []uint{}
			for i := range 3 {
				arg := args[i]
				index := slices.IndexFunc(variables, func(variable QCVariable) bool {
					return variable.Name == arg
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
