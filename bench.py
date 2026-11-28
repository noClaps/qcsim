import json
from argparse import ArgumentParser
from pathlib import Path
from subprocess import run
from typing import Literal


def cargo_build(name: str):
    run(
        ["cargo", "build", "--release", "--example", name],
        check=True,
        capture_output=True,
    )


def hyperfine(name: str, backend: Literal["qcsim", "qiskit"], warmup=10):
    if backend == "qiskit":
        prog = f"uv run qiskit/{name}.py"
    else:
        prog = f"./target/release/examples/{name}"

    data = run(
        [
            "hyperfine",
            prog,
            f"--warmup={warmup}",
            "--export-json=-",
            "--style=none",
        ],
        check=True,
        capture_output=True,
    ).stdout
    results = json.loads(data)["results"][0]
    return results


def print_heading(heading):
    print(f"\x1b[1;4m=== {heading} ===\x1b[0m")


ap = ArgumentParser()
ap.add_argument("-b", "--backend", default="qcsim", choices=["qcsim", "qiskit"])
ap.add_argument(
    "-p", "--program", default="both", choices=["grover", "coin-flip", "both"]
)
args = ap.parse_args()

if args.program == "grover" or args.program == "both":
    prev = 18
    if args.backend == "qiskit":
        file = Path("qiskit/grover.py")
    else:
        file = Path("examples/grover.rs")

    print_heading(f"grover ({args.backend})")
    print("| Qubits | Time | Memory |")
    print("| ------ | ---- | ------ |")

    for i in range(12, prev + 1):
        file.write_text(file.read_text().replace(f"{prev}", f"{i}", 1))
        prev = i

        cargo_build("grover")
        results = hyperfine("grover", args.backend)
        summary = results["summary"]
        mem_peak_res = summary["memory_peak_resident"]
        time_wall_clock = summary["time_wall_clock"]
        mem, mem_stddev, mem_unit = (
            round(mem_peak_res["mean"] / 1024 / 1024, 1),
            round(mem_peak_res["stddev"] / 1024 / 1024, 1),
            "MiB",
        )
        time, time_stddev, time_unit = (
            round(time_wall_clock["mean"] * 1000, 1),
            round(time_wall_clock["stddev"] * 1000, 1),
            "ms",
        )
        print(
            f"| {i} | {time} ± {time_stddev} {time_unit} | {mem} ± {mem_stddev} {mem_unit} |"
        )

if args.program == "coin-flip" or args.program == "both":
    cargo_build("coin-flip")
    results = hyperfine("coin-flip", args.backend)
    summary = results["summary"]
    mem_peak_res = summary["memory_peak_resident"]
    time_wall_clock = summary["time_wall_clock"]
    mem, mem_stddev, mem_unit = (
        round(mem_peak_res["mean"] / 1024 / 1024, 1),
        round(mem_peak_res["stddev"] / 1024 / 1024, 1),
        "MiB",
    )
    time, time_stddev, time_unit = (
        round(time_wall_clock["mean"] * 1000, 1),
        round(time_wall_clock["stddev"] * 1000, 1),
        "ms",
    )

    print_heading(f"coin-flip ({args.backend})")
    print("| Time | Memory |")
    print("| ---- | ------ |")
    print(f"| {time} ± {time_stddev} {time_unit} | {mem} ± {mem_stddev} {mem_unit} |")
