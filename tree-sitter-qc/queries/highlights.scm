
[
    "("
    ")"
] @punctuation.bracket

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
    "sin"
    "cos"
    "tan"
    "root"
    "measure"
    "x"
    "y"
    "z"
    "hadamard"
    "phase"
    "pi_8"
    "cnot"
    "cz"
    "swap"
    "toffoli"
] @function

[
    (pi)
    (euler)
    (imag)
] @constant

(comment) @comment
