package qclang

import (
	"encoding/json"
	"fmt"
	"log"
	"math"
	"math/cmplx"
	"slices"
	"strconv"
	"strings"

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

func (q *QCLang) Parse() {
	parser := tree_sitter.NewParser()
	defer parser.Close()
	parser.SetLanguage(tree_sitter.NewLanguage(tree_sitter_qc.Language()))

	tree := parser.Parse([]byte(q.Input), nil)
	defer tree.Close()

	nodes := []tree_sitter.Node{*tree.RootNode()}
	q.parseTree(nodes, 0)
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

func (q *QCLang) parseTree(nodes []tree_sitter.Node, depth uint) {
	for _, node := range nodes {
		cursor := node.Walk()
		children := node.Children(cursor)
		switch node.GrammarName() {
		case "variable_declaration":
			q.addVariable(children)
		case "measure":
			q.addInstr(Measure, children, "uint")
		case "pauli_x":
			q.addInstr(PauliX, children, "var_name")
		case "pauli_y":
			q.addInstr(PauliY, children, "var_name")
		case "pauli_z":
			q.addInstr(PauliZ, children, "var_name")
		case "hadamard":
			q.addInstr(Hadamard, children, "var_name")
		case "phase":
			q.addInstr(Phase, children, "var_name")
		case "pi_by_8":
			q.addInstr(PiBy8, children, "var_name")
		case "controlled_not":
			q.addInstr(ControlledNot, children, "var_name")
		case "controlled_z":
			q.addInstr(ControlledZ, children, "var_name")
		case "swap":
			q.addInstr(Swap, children, "var_name")
		case "toffoli":
			q.addInstr(Toffoli, children, "var_name")
		default:
			if node.ChildCount() > 0 {
				q.parseTree(children, depth+1)
			}
		}
	}
}

func (q *QCLang) addVariable(children []tree_sitter.Node) error {
	varName := q.getSlice(children[0].Range())

	varExpr := children[2]
	cursor := varExpr.Walk()
	children = varExpr.Children(cursor)

	var qubitZero complex128 = 0
	for _, child := range children {
		if child.GrammarName() != "qubit_zero" {
			continue
		}

		cursor = child.Walk()
		nodeChildren := child.Children(cursor)
		qubit, err := q.getNumValue(nodeChildren)
		if err != nil {
			return err
		}
		qubitZero = qubit
	}
	var qubitOne complex128 = 0
	for _, child := range children {
		if child.GrammarName() != "qubit_one" {
			continue
		}

		cursor = child.Walk()
		nodeChildren := child.Children(cursor)
		qubit, err := q.getNumValue(nodeChildren)
		if err != nil {
			return err
		}
		qubitOne = qubit
	}

	newQubit, err := qubit.New(qubitZero, qubitOne)
	if err != nil {
		return err
	}
	q.Variables = append(q.Variables, QCVariable{varName, *newQubit})
	return nil
}

func (q *QCLang) getSlice(nodeRange tree_sitter.Range) string {
	start := nodeRange.StartPoint
	end := nodeRange.EndPoint
	lines := slices.Collect(strings.Lines(q.Input))

	return lines[start.Row][start.Column:end.Column]
}

func (q *QCLang) getNumValue(children []tree_sitter.Node) (complex128, error) {
	switch {
	case q.getSlice(children[0].Range()) == "|0>":
		return 1, nil
	case q.getSlice(children[0].Range()) == "|1>":
		return 1, nil
	default:
		cursor := 0
		for children[cursor].GrammarName() == "(" {
			cursor++
		}
		return q.parseExpr(children[cursor])
	}
}

func (q *QCLang) parseExpr(node tree_sitter.Node) (complex128, error) {
	switch node.GrammarName() {
	case "number":
		return strconv.ParseComplex(q.getSlice(node.Range()), 128)

	// constants
	case "pi":
		return math.Pi, nil
	case "imag":
		return complex(0, 1), nil
	case "euler":
		return math.E, nil

	// binary expressions
	case "add":
		arg1, err := q.parseExpr(*node.ChildByFieldName("arg1"))
		if err != nil {
			return 0, err
		}
		arg2, err := q.parseExpr(*node.ChildByFieldName("arg2"))
		if err != nil {
			return 0, err
		}
		return arg1 + arg2, nil
	case "sub":
		arg1, err := q.parseExpr(*node.ChildByFieldName("arg1"))
		if err != nil {
			return 0, err
		}
		arg2, err := q.parseExpr(*node.ChildByFieldName("arg2"))
		if err != nil {
			return 0, err
		}
		return arg1 - arg2, nil
	case "mul":
		arg1, err := q.parseExpr(*node.ChildByFieldName("arg1"))
		if err != nil {
			return 0, err
		}
		arg2, err := q.parseExpr(*node.ChildByFieldName("arg2"))
		if err != nil {
			return 0, err
		}
		return arg1 * arg2, nil
	case "div":
		arg1, err := q.parseExpr(*node.ChildByFieldName("arg1"))
		if err != nil {
			return 0, err
		}
		arg2, err := q.parseExpr(*node.ChildByFieldName("arg2"))
		if err != nil {
			return 0, err
		}
		return arg1 / arg2, nil
	case "exp":
		arg1, err := q.parseExpr(*node.ChildByFieldName("arg1"))
		if err != nil {
			return 0, err
		}
		arg2, err := q.parseExpr(*node.ChildByFieldName("arg2"))
		if err != nil {
			return 0, err
		}
		return cmplx.Pow(arg1, arg2), nil

	// other functions
	case "sin":
		arg, err := q.parseExpr(*node.ChildByFieldName("arg"))
		if err != nil {
			return 0, err
		}
		return cmplx.Sin(arg), nil
	case "cos":
		arg, err := q.parseExpr(*node.ChildByFieldName("arg"))
		if err != nil {
			return 0, err
		}
		return cmplx.Cos(arg), nil
	case "tan":
		arg, err := q.parseExpr(*node.ChildByFieldName("arg"))
		if err != nil {
			return 0, err
		}
		return cmplx.Tan(arg), nil
	case "root":
		arg1, err := q.parseExpr(*node.ChildByFieldName("arg1"))
		if err != nil {
			return 0, err
		}
		arg2, err := q.parseExpr(*node.ChildByFieldName("arg2"))
		if err != nil {
			return 0, err
		}
		return cmplx.Pow(arg1, 1/arg2), nil
	default:
		return q.parseExpr(*node.NextSibling())
	}
}

func (q *QCLang) addInstr(instr Instruction, children []tree_sitter.Node, grammarName string) {
	args := slices.Collect(func(yield func(string) bool) {
		for _, node := range children {
			if node.GrammarName() == grammarName {
				name := q.getSlice(node.Range())
				switch grammarName {
				case "var_name":
					if !yield(name) {
						return
					}
				case "uint":
					if _, err := strconv.ParseUint(name, 10, 0); err != nil {
						return
					}
					if !yield(name) {
						return
					}
				}
			}
		}
	})

	q.Instructions = append(q.Instructions, QCFunction{instr, args})
}

func formatOutputs(outputs map[string]uint) string {
	b, err := json.MarshalIndent(outputs, "", "")
	if err != nil {
		log.Fatalln("[ERROR]", err)
	}
	return string(b[2 : len(b)-2])
}
