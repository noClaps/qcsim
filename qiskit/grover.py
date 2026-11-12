import numpy as np
from qiskit import QuantumCircuit
from qiskit.circuit.library import UnitaryGate
from qiskit.primitives import StatevectorSampler

secret_val = (1 << 12) - 1


def check(x: int):
    return secret_val == x


num_qubits = int(np.log2(secret_val)) + 1
print(f"num qubits: {num_qubits}")
iterations = int(np.ceil(np.pi / 4 * np.sqrt(num_qubits)))
print(f"iterations: {iterations}")

u_omega = UnitaryGate(np.diag([(-1) ** check(i) for i in range(1 << num_qubits)]))
u_s = np.zeros((1 << num_qubits, 1 << num_qubits))
u_s[0, 0] = 1
u_s = u_s * 2 - np.eye(1 << num_qubits)
u_s = UnitaryGate(u_s)

qc = QuantumCircuit(num_qubits)
qc.h(range(num_qubits))
for i in range(iterations):
    print(f"grover iteration {i + 1}")
    qc.unitary(u_omega, range(num_qubits))
    qc.h(range(num_qubits))
    qc.unitary(u_s, range(num_qubits))
    qc.h(range(num_qubits))

qc.measure_all()

result = StatevectorSampler(default_shots=10_000).run([qc]).result()
data: dict[int, int] = result[0].data["meas"].get_int_counts()
val = max(data.items(), key=lambda item: item[1])[0]

assert check(val), f"Grover's algorithm failed! Expected: {secret_val}, received: {val}"
print(f"val: {val}")
