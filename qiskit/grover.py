import numpy as np
from qiskit import QuantumCircuit
from qiskit.circuit.library import DiagonalGate
from qiskit_aer.primitives import SamplerV2

secret_val = (1 << 18) - 1


def check(x: int):
    return secret_val == x


num_qubits = int(np.log2(secret_val)) + 1
print(f"num qubits: {num_qubits}")
iterations = int(np.ceil(np.pi / 4 * np.sqrt(2**num_qubits)))
print(f"iterations: {iterations}")

u_omega = DiagonalGate([(-1) ** check(i) for i in range(1 << num_qubits)])
u_s = DiagonalGate([1] + [-1] * ((1 << num_qubits) - 1))

qc = QuantumCircuit(num_qubits)
qc.h(range(num_qubits))
for i in range(iterations):
    qc.append(u_omega, range(num_qubits))
    qc.h(range(num_qubits))
    qc.append(u_s, range(num_qubits))
    qc.h(range(num_qubits))

qc.measure_all()

result = SamplerV2(default_shots=10_000).run([qc]).result()
data: dict[int, int] = result[0].data["meas"].get_int_counts()
val = max(data.items(), key=lambda item: item[1])[0]

assert check(val), f"Grover's algorithm failed! Expected: {secret_val}, received: {val}"
print(f"val: {val}")
