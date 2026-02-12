#!/usr/bin/env python3
"""
Provncloud SDK Monorepo Manager

Unified interface for building and testing all SDK components.
Usage:
    python manage.py build   # Build all SDKs
    python manage.py test    # Test all SDKs
    python manage.py clean   # Clean all artifacts
"""

import os
import subprocess
import sys
import shutil

ROOT_DIR = os.path.dirname(os.path.abspath(__file__))

def run_cmd(cmd, cwd=None, ignore_error=False):
    """Run a shell command"""
    print(f"🔹 Running: {cmd} (in {cwd or '.'})")
    try:
        subprocess.check_call(cmd, shell=True, cwd=cwd)
    except subprocess.CalledProcessError:
        print(f"❌ Command failed: {cmd}")
        if not ignore_error:
            sys.exit(1)

def build_rust():
    print("\n🦀 Building Rust Core...")
    run_cmd("cargo build --release", cwd=os.path.join(ROOT_DIR, "rust"))

def build_wasm():
    print("\n🕸️ Building Rust WASM...")
    run_cmd("wasm-pack build --target nodejs --scope provn", cwd=os.path.join(ROOT_DIR, "rust-wasm"))
    # Also build for web if needed, but nodejs is primary for TS SDK build

def build_typescript():
    print("\n📘 Building TypeScript SDK...")
    ts_dir = os.path.join(ROOT_DIR, "typescript")
    run_cmd("npm install", cwd=ts_dir)
    run_cmd("npm run build", cwd=ts_dir)

def build_python():
    print("\n🐍 Building Python SDK...")
    py_dir = os.path.join(ROOT_DIR, "python")
    # Ensure maturin is installed
    run_cmd("pip install maturin", cwd=py_dir, ignore_error=True)
    run_cmd("maturin develop --release", cwd=py_dir)

def build_go():
    print("\n🐹 Building Go SDK...")
    run_cmd("go build ./...", cwd=os.path.join(ROOT_DIR, "go"))

def build_java():
    print("\n☕ Building Java SDK...")
    if shutil.which("mvn"):
        run_cmd("mvn compile", cwd=os.path.join(ROOT_DIR, "java"))
    else:
        print("⚠️ Maven (mvn) not found, skipping Java build.")

def test():
    print("\n🧪 Testing All Components...")
    
    print(">> Rust Tests")
    run_cmd("cargo test", cwd=os.path.join(ROOT_DIR, "rust"))
    
    print(">> TypeScript Tests")
    run_cmd("npm test", cwd=os.path.join(ROOT_DIR, "typescript"))
    
    print(">> Python Tests")
    run_cmd("python test_sdk_verify.py", cwd=ROOT_DIR)
    
    print(">> Go Tests")
    run_cmd("go test ./...", cwd=os.path.join(ROOT_DIR, "go"))
    
    print("\n✅ All Tests Passed!")

def clean():
    print("\n🧹 Cleaning Artifacts...")
    run_cmd("cargo clean", cwd=os.path.join(ROOT_DIR, "rust"))
    run_cmd("cargo clean", cwd=os.path.join(ROOT_DIR, "rust-wasm"))
    
    ts_dist = os.path.join(ROOT_DIR, "typescript", "dist")
    if os.path.exists(ts_dist): shutil.rmtree(ts_dist)
    
    py_target = os.path.join(ROOT_DIR, "python", "target")
    if os.path.exists(py_target): shutil.rmtree(py_target)
    
    print("✨ Clean complete.")

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
        print("\n✅ Build Complete!")
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
