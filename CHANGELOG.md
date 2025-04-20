# Changelog

## v0.2.0

### Breaking changes

- Update tree-sitter queries to better accommodate syntax highlighting. This changes the grammar too, hence why this is a breaking change, though none of the functionality of QCSim has changed.

### New features

- Add Zed extension. To install it:
  1. Clone this repository.
  2. Open the extensions menu in Zed.
  3. Click on Install Dev Extension.
  4. Navigate to the `qc-zed` directory in the cloned repository.
  5. You should now have syntax highlighting for QC files.

## v0.1.1

- Return error from recursive `parseTree()` call. Now errors are properly returned during parsing, they weren't before.
- Add string representation of qubit for use in logging. Qubits will now look like `([real] + [imaginary]i) |0> + ([real] + [imaginary]i) |1>` if printed using `fmt.Printf()` or `log.Printf()`.
- Only check for qubit normalisation on variable declaration. Qubits that are already in the system will not be checked for normalisation, as some combinations of gates can cause both `|0>` and `|1>` values of qubit to go to 0, which was previously causing issues. This closes #1.

## v0.1.0

Initial release!
