#!/usr/bin/env python3
import subprocess
import sys
import json
import urllib.request
import time
import os

# RinCLI Dev Harness for testing across models and agents.
# Standardizes validations of CLI outputs and API behaviors.

RINCLI_BIN = "./target/debug/rincli"
if os.name == "nt":
    RINCLI_BIN += ".exe"

def run_command(args):
    print(f"[HARNESS] Running: {' '.join(args)}")
    try:
        res = subprocess.run(args, capture_output=True, text=True)
        return res
    except OSError as e:
        print(f"[HARNESS] Failed to run command {' '.join(args)}: {e}")
        return subprocess.CompletedProcess(
            args=args,
            returncode=127,
            stdout="",
            stderr=str(e)
        )

def test_cli():
    print("[HARNESS] --- CLI Verification ---")
    
    # Test --help
    r = run_command([RINCLI_BIN, "--help"])
    if r.returncode != 0:
        print("FAIL: --help failed")
        return False
        
    # Test doctor
    r = run_command([RINCLI_BIN, "doctor"])
    if r.returncode != 0:
        print("FAIL: doctor failed")
        return False
    
    # Test config get
    r = run_command([RINCLI_BIN, "config", "get"])
    if r.returncode != 0:
        print("FAIL: config get failed")
        return False

    # Test search plain
    r = run_command([RINCLI_BIN, "search", "qwen", "--plain"])
    if r.returncode != 0 or "qwen" not in r.stdout.lower():
        print("FAIL: search plain failed")
        return False

    # Test search json
    r = run_command([RINCLI_BIN, "search", "qwen", "--json"])
    try:
        data = json.loads(r.stdout.strip())
        if "query" not in data:
            print("FAIL: search json missing query field")
            return False
    except Exception as e:
        print(f"FAIL: search json invalid json output: {e}")
        return False

    print("PASS: CLI commands verified successfully.")
    return True

def test_api():
    print("[HARNESS] --- API Verification ---")
    print("Skipping active runtime API checks in Phase 1 (Stubs mode).")
    # In future phases, this will run 'rincli run' and query localhost:6767/v1/chat/completions
    return True

def main():
    if not os.path.exists(RINCLI_BIN):
        # Try to build
        print("[HARNESS] Binary not found. Trying to build...")
        user_profile = os.environ.get("USERPROFILE", "")
        possible_paths = [
            os.path.join(user_profile, ".cargo", "bin", "cargo.exe"),
            os.path.join(user_profile, "scoop", "persist", "rustup", ".cargo", "bin", "cargo.exe"),
            os.path.join(user_profile, "scoop", "apps", "rustup", "current", ".cargo", "bin", "cargo.exe"),
        ]
        cargo_home = os.environ.get("CARGO_HOME")
        if cargo_home:
            possible_paths.insert(0, os.path.join(cargo_home, "bin", "cargo.exe"))

        cargo_path = "cargo"
        for path in possible_paths:
            if os.path.exists(path):
                cargo_path = path
                break
        
        try:
            r = subprocess.run([cargo_path, "build"])
            if r.returncode != 0:
                print("FAIL: Cargo build failed. Cannot run harness.")
                sys.exit(1)
        except OSError as e:
            print(f"FAIL: Cargo executable not found or failed to execute: {e}")
            print("Please ensure Rust and Cargo are installed and in your PATH.")
            sys.exit(1)

    cli_ok = test_cli()
    api_ok = test_api()

    if cli_ok and api_ok:
        print("[HARNESS] ALL TESTS PASSED.")
        sys.exit(0)
    else:
        print("[HARNESS] SOME TESTS FAILED.")
        sys.exit(1)

if __name__ == "__main__":
    main()
