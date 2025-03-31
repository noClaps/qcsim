# Changelog

## v0.1.1

- Return error from recursive `parseTree()` call. Now errors are properly returned during parsing, they weren't before.
- Add string representation of qubit for use in logging. Qubits will now look like `([real] + [imaginary]i) |0> + ([real] + [imaginary]i) |1>` if printed using `fmt.Printf()` or `log.Printf()`.
- Only check for qubit normalisation on variable declaration. Qubits that are already in the system will not be checked for normalisation, as some combinations of gates can cause both `|0>` and `|1>` values of qubit to go to 0, which was previously causing issues. This closes #1.

## v0.1.0

Initial release!
