# Notes

These are notes for things that I found through various methods, whether on my own or by going back and forth with an LLM. This is not a complete history of all the changes I've made to the codebase as I do not remember most of the details, but it should be somewhat accurate from here on out.

## Qiskit Aer

This was suggested to me by Claude when benchmarking the Grover implementations. [Qiskit Aer](https://qiskit.github.io/qiskit-aer/) is a higher performance quantum computer simulator, with its main usage being through `SamplerV2`. While I don't have exact numbers, this pretty massively sped up the Qiskit version of Grover. The other big optimisation was using `DiagonalGate` instead of `UnitaryGate`, which gave the same (relative) benefits as the diagonal matrix implementation in QCSim.

Interestingly, it didn't provide the same benefits in coin-flip, which is likely due to the overhead of having to pull in a whole other library.

## Improved sampling

The way the `sample` function worked before was that it would generate a random number between 0 and 1, then go through each of the entries in the state vector, accumulating the norm squares until it was greater than or equal to the random number, and then returned the index at the end. And it would repeat all of that $N$ times, where $N$ is the number of samples being taken.

```
fn measure():
  rand = random_f64() # 0...1
  total = 0
  for i in state_vector:
    amp = state_vector[i]
    total = total + norm_square(amp)
    if total >= rand:
      return i
  return state_vector.len - 1

results = [:]
repeat N times:
  outcome = measure()
  if outcome not in results:
    results[outcome] = 1
  else:
    results[outcome] = results[outcome] + 1
```

This is, of course, $O(N \times 2^n)$ where $n$ is the number of qubits. I originally didn't think it was that big of a deal, since it didn't show up very prominently in my profiling, but Claude suggested it might be a place to try looking. Its suggestion was to instead precalculate the cumulative distribution that I was doing inside the `measure()` function, and just index into that directly.

```
cdf = []
total = 0
for i in state_vector:
  amp = state_vector[i]
  total = total + norm_square(amp)
  cdf[i] = total

results = [:]
repeat N times:
  rand = random_f64()
  idx = binary_search(cdf, fn(i): cdf[i] <= rand)
  if idx not in results:
    results[idx] = 0
  else:
    results[idx] = results[idx] + 1
```

This brings down the complexity to $O(2^n + N \times n)$, with the $2^n$ coming from the `for i in state_vector` loop, and the $N \times n$ coming from the `repeat N times` loop and the `binary_search` inside. Of course, this is far better than $O(N \times 2^n)$.
