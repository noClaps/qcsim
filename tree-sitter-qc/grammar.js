/**
 * @file The instructions language for QCSim
 * @author noClaps <contact@zerolimits.dev>
 * @license 0BSD
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

module.exports = grammar({
  name: "qc",

  rules: {
    source_file: ($) => seq(repeat1($.variable_declaration), repeat($._qc_fn)),

    // Variable declaration
    variable_declaration: ($) => seq($.var_name, "=", $.var_expr),
    var_name: (_) => /[a-zA-Z]\w*/,
    var_expr: ($) =>
      choice(
        seq($.qubit_zero, optional(seq(",", $.qubit_one))),
        seq($.qubit_one, optional(seq(",", $.qubit_zero))),
      ),
    number: (_) => /-?\d+(\.\d+)?/,
    uint: (_) => /\d+/,

    qubit_zero: ($) => seq(optional($._expr), "|0>"),
    qubit_one: ($) => seq(optional($._expr), "|1>"),

    _expr: ($) =>
      choice(
        seq("(", $._expr, ")"),
        $._binary_expr,
        $._var_fn,
        $.number,
        $._constant,
      ),

    _binary_expr: ($) => seq(choice($.add, $.sub, $.mul, $.div, $.exp)),
    add: ($) =>
      prec.left(1, seq(field("arg1", $._expr), "+", field("arg2", $._expr))),
    sub: ($) =>
      prec.left(1, seq(field("arg1", $._expr), "-", field("arg2", $._expr))),
    mul: ($) =>
      prec.left(2, seq(field("arg1", $._expr), "*", field("arg2", $._expr))),
    div: ($) =>
      prec.left(2, seq(field("arg1", $._expr), "/", field("arg2", $._expr))),
    exp: ($) =>
      prec.left(3, seq(field("arg1", $._expr), "^", field("arg2", $._expr))),

    _var_fn: ($) => choice($.sin, $.cos, $.tan, $.root),
    sin: ($) => seq("sin(", field("arg", $._expr), ")"),
    cos: ($) => seq("cos(", field("arg", $._expr), ")"),
    tan: ($) => seq("tan(", field("arg", $._expr), ")"),
    root: ($) =>
      seq("root(", field("arg1", $._expr), ",", field("arg2", $._expr), ")"),

    _constant: ($) => choice($.pi, $.imag, $.euler),
    pi: (_) => "PI",
    imag: (_) => "I",
    euler: (_) => "E",

    // QC function call
    _qc_fn: ($) =>
      choice(
        $.measure,
        $.pauli_x,
        $.pauli_y,
        $.pauli_z,
        $.hadamard,
        $.phase,
        $.pi_by_8,
        $.controlled_not,
        $.controlled_z,
        $.swap,
        $.toffoli,
      ),
    measure: ($) => seq("measure(", optional($.uint), ")"),
    pauli_x: ($) => seq("x(", $.var_name, ")"),
    pauli_y: ($) => seq("y(", $.var_name, ")"),
    pauli_z: ($) => seq("z(", $.var_name, ")"),
    hadamard: ($) => seq("hadamard(", $.var_name, ")"),
    phase: ($) => seq("phase(", $.var_name, ")"),
    pi_by_8: ($) => seq("pi_8(", $.var_name, ")"),
    controlled_not: ($) => seq("cnot(", $.var_name, ",", $.var_name, ")"),
    controlled_z: ($) => seq("cz(", $.var_name, ",", $.var_name, ")"),
    swap: ($) => seq("swap(", $.var_name, ",", $.var_name, ")"),
    toffoli: ($) =>
      seq("toffoli(", $.var_name, ",", $.var_name, ",", $.var_name, ")"),

    // Extra
    _comment: (_) => seq("//", /.*/),
  },

  extras: ($) => [/\s/, $._comment],
});
