# Notes

These are notes for things that I found through various methods, whether on my own or by going back and forth with an LLM. This is not a complete history of all the changes I've made to the codebase as I do not remember most of the details, but it should be somewhat accurate from here on out.

## Qiskit Aer

This was suggested to me by Claude when benchmarking the Grover implementations. [Qiskit Aer](https://qiskit.github.io/qiskit-aer/) is a higher performance quantum computer simulator, with its main usage being through `SamplerV2`. While I don't have exact numbers, this pretty massively sped up the Qiskit version of Grover. The other big optimisation was using `DiagonalGate` instead of `UnitaryGate`, which gave the same (relative) benefits as the diagonal matrix implementation in QCSim.

Interestingly, it didn't provide the same benefits in coin-flip, which is likely due to the overhead of having to pull in a whole other library.
