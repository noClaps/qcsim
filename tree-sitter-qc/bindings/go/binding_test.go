package tree_sitter_qc_test

import (
	"testing"

	tree_sitter_qc "github.com/noclaps/qcsim/tree-sitter-qc/bindings/go"
	tree_sitter "github.com/tree-sitter/go-tree-sitter"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_qc.Language())
	if language == nil {
		t.Errorf("Error loading QC grammar")
	}
}
