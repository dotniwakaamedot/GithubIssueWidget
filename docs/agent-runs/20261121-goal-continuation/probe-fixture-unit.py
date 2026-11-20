"""Execute a no-network synthetic Rust test in a single-process Job."""
import hashlib
import os
from pathlib import Path
import sys

run_dir = Path(__file__).resolve().parent
sys.path.insert(0, str(run_dir.parent / "20261121-output-truncation"))
import job_process

exe = run_dir.parent / "20261120-current-review" / "live-fixture-d295bc126897471ba7ddcc661216143b" / "src-tauri" / "target" / "debug" / "deps" / "github_issue_widget-fc25f1eb6ae85386.exe"
expected = "193B97EDA5F965077229EE5F8D642A88BBB092BFC46470CA703FBFD3E1756EE1"
if hashlib.sha256(exe.read_bytes()).hexdigest().upper() != expected:
    raise SystemExit("fixture hash mismatch")
system_root = os.environ.get("SystemRoot")
if not system_root:
    raise SystemExit("SystemRoot unavailable")
temp_dir = run_dir / "temp-fixture-unit"
temp_dir.mkdir(exist_ok=True)
env = {"SystemRoot": system_root, "TEMP": str(temp_dir), "TMP": str(temp_dir)}
name = "creation::tests::blank_title_is_rejected_without_network"
result = job_process.run([str(exe), "--exact", name, "--test-threads=1"], timeout=10, env=env, active_process_limit=1)
passed = (name + " ... ok").encode() in result.stdout and b"1 passed; 0 failed" in result.stdout
print(f"exit={result.returncode}; selected_test_passed={passed}")
if result.returncode != 0 or not passed:
    # The selected test and environment contain no real credentials or issue data.
    print(result.stdout.decode("utf-8", errors="replace"))
    print(result.stderr.decode("utf-8", errors="replace"))
    raise SystemExit(1)
