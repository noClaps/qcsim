build:
	@cd tree-sitter-qc && tree-sitter generate
	@go build

ext:
	@cp tree-sitter-qc/queries/highlights.scm qc-zed/languages/qc/
