q1 = (1/root(2,2)) |0>, (1/root(2,2)) |1>
q2 = |0>
q3 = |1>

hadamard(q1)
x(q1)
x(q2)
toffoli(q1, q2, q3)
measure(100000) // returns "110"
