from qiskit import QuantumCircuit
from qiskit.primitives import StatevectorSampler

qc = QuantumCircuit(1)
qc.h(0)
qc.measure_all()

result = StatevectorSampler(default_shots=10000).run([qc]).result()
data: dict[int, int] = result[0].data["meas"].get_counts()

print(data)
