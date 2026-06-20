#!/usr/bin/env python3
"""
Provncloud SDK Monorepo Manager

Unified interface for building and testing all SDK components.
Usage:
    python scripts/build-all.py build
    python scripts/build-all.py test
    python scripts/build-all.py clean
"""

import io
import os
import pathlib
import shutil
import subprocess
import sys

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")

ROOT_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def run_cmd(cmd, cwd=None):
    """Run a shell command and fail fast if it errors."""
    print(f"Running: {cmd} (in {cwd or '.'})")
    subprocess.check_call(cmd, shell=True, cwd=cwd)


def build_and_install_python_wheel(py_dir):
    """Build a wheel for the active interpreter and install it."""
    wheels_dir = pathlib.Path(py_dir) / "target" / "wheels"
    if wheels_dir.exists():
        shutil.rmtree(wheels_dir)

    run_cmd(f'maturin build --release --interpreter "{sys.executable}"', cwd=py_dir)

    wheels = sorted(wheels_dir.glob("*.whl"), key=lambda p: p.stat().st_mtime, reverse=True)
    if not wheels:
        raise RuntimeError(f"No wheels found in {wheels_dir}")

    wheel_path = wheels[0]
    run_cmd(
        f'"{sys.executable}" -m pip install --force-reinstall "{wheel_path}"',
        cwd=py_dir,
    )


def build_rust():
    print("\nBuilding Rust Core...")
    run_cmd("cargo build --release", cwd=os.path.join(ROOT_DIR, "rust"))


def build_wasm():
    print("\nBuilding Rust WASM...")
    run_cmd(
        "wasm-pack build --target nodejs --scope provn",
        cwd=os.path.join(ROOT_DIR, "rust-wasm"),
    )


def build_typescript():
    print("\nBuilding TypeScript SDK...")
    ts_dir = os.path.join(ROOT_DIR, "typescript")
    run_cmd("npm ci", cwd=ts_dir)
    run_cmd("npm run build", cwd=ts_dir)


def build_python():
    print("\nBuilding Python SDK...")
    py_dir = os.path.join(ROOT_DIR, "python")
    run_cmd(f'"{sys.executable}" -m pip install maturin', cwd=py_dir)
    build_and_install_python_wheel(py_dir)


def build_go():
    print("\nBuilding Go SDK...")
    run_cmd("go build ./...", cwd=os.path.join(ROOT_DIR, "go"))


def build_java():
    print("\nBuilding Java SDK...")
    if shutil.which("mvn"):
        run_cmd("mvn compile", cwd=os.path.join(ROOT_DIR, "java"))
    else:
        print("WARNING: Maven (mvn) not found, skipping Java build.")


def test():
    print("\nTesting All Components...")
    skipped = []

    print(">> Rust Tests")
    run_cmd("cargo test", cwd=os.path.join(ROOT_DIR, "rust"))

    print(">> TypeScript Tests")
    run_cmd("npm test", cwd=os.path.join(ROOT_DIR, "typescript"))

    print(">> Python Tests")
    py_dir = os.path.join(ROOT_DIR, "python")
    build_and_install_python_wheel(py_dir)
    run_cmd(f'"{sys.executable}" -m pytest tests/ -v', cwd=py_dir)

    print(">> Go Tests")
    run_cmd("go test ./...", cwd=os.path.join(ROOT_DIR, "go"))

    print(">> Java Tests")
    if shutil.which("mvn"):
        run_cmd("mvn test", cwd=os.path.join(ROOT_DIR, "java"))
    else:
        skipped.append("java")
        print("WARNING: Maven (mvn) not found, skipping Java tests.")

    if skipped:
        print(f"\nAll available tests passed. Skipped: {', '.join(skipped)}")
    else:
        print("\nAll tests passed.")


def clean():
    print("\nCleaning Artifacts...")
    run_cmd("cargo clean", cwd=os.path.join(ROOT_DIR, "rust"))
    run_cmd("cargo clean", cwd=os.path.join(ROOT_DIR, "rust-wasm"))

    ts_dist = os.path.join(ROOT_DIR, "typescript", "dist")
    if os.path.exists(ts_dist):
        shutil.rmtree(ts_dist)

    py_target = os.path.join(ROOT_DIR, "python", "target")
    if os.path.exists(py_target):
        shutil.rmtree(py_target)

    print("Clean complete.")


def main():
    if len(sys.argv) < 2:
        print(__doc__)
        sys.exit(1)

    command = sys.argv[1]

    if command == "build":
        build_rust()
        build_wasm()
        build_typescript()
        build_python()
        build_go()
        build_java()
        print("\nBuild complete.")
    elif command == "test":
        test()
    elif command == "clean":
        clean()
    else:
        print(f"Unknown command: {command}")
        print(__doc__)
        sys.exit(1)


if __name__ == "__main__":
    main()
