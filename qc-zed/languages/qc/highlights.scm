[
    "("
    ")"
] @punctuation.bracket

[
    ","
] @punctuation.delimiter

[
    (number)
    (uint)
] @number

[
    "+"
    "-"
    "*"
    "/"
    "^"
    "="
] @operator

(var_name) @variable

[
    (sin)
    (cos)
    (tan)
    (root)
    (measure)
    (pauli_x)
    (pauli_y)
    (pauli_z)
    (hadamard)
    (phase)
    (pi_by_8)
    (controlled_not)
    (controlled_z)
    (swap)
    (toffoli)
] @function

[
    (pi)
    (euler)
    (imag)
] @constant

(comment) @comment

[
    "|0>"
    "|1>"
] @keyword
