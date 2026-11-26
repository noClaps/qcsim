import json
import sys
from pathlib import Path
from subprocess import run


def cargo_build(name):
    run(
        ["cargo", "build", "--release", "--example", name],
        check=True,
        capture_output=True,
    )


def hyperfine(name, warmup=10):
    data = run(
        [
            "hyperfine",
            f"./target/release/examples/{name}",
            f"uv run qiskit/{name}.py",
            f"--warmup={warmup}",
            "--export-json=-",
            "--style=none",
        ],
        check=True,
        capture_output=True,
    ).stdout
    results = json.loads(data)["results"]
    return results


def print_heading(heading):
    print(f"\x1b[1;4m=== {heading} ===\x1b[0m")


if "grover" in sys.argv or len(sys.argv) == 1:
    print_heading("grover")
    prev = 18
    qcsim = Path("examples/grover.rs")
    qiskit = Path("qiskit/grover.py")

    print("|           | QCSim | Qiskit |")
    print("| --------- | ----- | ------ |")
    for i in range(12, prev + 1):
        qcsim.write_text(qcsim.read_text().replace(f"{prev}", f"{i}", 1))
        qiskit.write_text(qiskit.read_text().replace(f"{prev}", f"{i}", 1))
        prev = i

        cargo_build("grover")
        results = hyperfine("grover")
        print(f"| {i} qubits |", end="")
        for r in results:
            summary = r["summary"]
            mem, mem_unit = summary["memory_peak_resident"]["mean"] / 1024 / 1024, "MiB"
            time, time_unit = summary["time_wall_clock"]["mean"], "s"
            if time < 10:
                time, time_unit = time * 1000, "ms"
            print(f" {time:.1f} {time_unit} ({mem:.1f} {mem_unit}) |", end="")
        print()

if "coin-flip" in sys.argv or len(sys.argv) == 1:
    print_heading("coin-flip")
    print("| QCSim | Qiskit |")
    print("| ----- | ------ |")
    print("|", end="")
    cargo_build("coin-flip")
    results = hyperfine("coin-flip")
    for r in results:
        summary = r["summary"]
        mem, mem_unit = summary["memory_peak_resident"]["mean"] / 1024 / 1024, "MiB"
        time, time_unit = summary["time_wall_clock"]["mean"], "s"
        if time < 10:
            time, time_unit = time * 1000, "ms"
        print(f" {time:.1f} {time_unit} ({mem:.1f} {mem_unit}) |", end="")
    print()
