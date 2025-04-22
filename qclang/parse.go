package qclang

import (
	"fmt"
	"math"
	"math/cmplx"
	"slices"
	"strconv"
	"strings"

	"github.com/qcsim/qcsim/qubit"
	tree_sitter "github.com/tree-sitter/go-tree-sitter"
)

func (q *qcLang) parseTree(nodes []tree_sitter.Node, depth uint) error {
	for _, node := range nodes {
		cursor := node.Walk()
		children := node.Children(cursor)
		switch node.GrammarName() {
		case "variable_declaration":
			if err := q.addVariable(children); err != nil {
				return err
			}
		case "measure":
			q.addInstr(measure, children, "uint")
		case "pauli_x":
			q.addInstr(pauliX, children, "var_name")
		case "pauli_y":
			q.addInstr(pauliY, children, "var_name")
		case "pauli_z":
			q.addInstr(pauliZ, children, "var_name")
		case "hadamard":
			q.addInstr(hadamard, children, "var_name")
		case "phase":
			q.addInstr(phase, children, "var_name")
		case "pi_by_8":
			q.addInstr(piBy8, children, "var_name")
		case "controlled_not":
			q.addInstr(controlledNot, children, "var_name")
		case "controlled_z":
			q.addInstr(controlledZ, children, "var_name")
		case "swap":
			q.addInstr(swap, children, "var_name")
		case "toffoli":
			q.addInstr(toffoli, children, "var_name")
		default:
			if node.ChildCount() > 0 {
				if err := q.parseTree(children, depth+1); err != nil {
					return err
				}
			}
		}
	}

	return nil
}

func (q *qcLang) addVariable(children []tree_sitter.Node) error {
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

	newQubit := qubit.New(qubitZero, qubitOne)
	if !newQubit.IsNormalised() {
		return fmt.Errorf("Qubit is not normalised: %s = %+v", varName, newQubit)
	}

	q.variables = append(q.variables, qcVariable{varName, newQubit})
	return nil
}

func (q *qcLang) addInstr(instr gate, children []tree_sitter.Node, grammarName string) {
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

	q.instructions = append(q.instructions, qcFunction{instr, args})
}

func (q *qcLang) getSlice(nodeRange tree_sitter.Range) string {
	start := nodeRange.StartPoint
	end := nodeRange.EndPoint
	lines := slices.Collect(strings.Lines(q.input))

	return lines[start.Row][start.Column:end.Column]
}

func (q *qcLang) getNumValue(children []tree_sitter.Node) (complex128, error) {
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

func (q *qcLang) parseExpr(node tree_sitter.Node) (complex128, error) {
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
