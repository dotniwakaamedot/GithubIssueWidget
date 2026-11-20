"""List one test under a single-process Job; performs no test or network operation."""
import hashlib
from pathlib import Path
import sys

run_dir = Path(__file__).resolve().parent
sys.path.insert(0, str(run_dir.parent / "20261121-output-truncation"))
import job_process

exe = run_dir.parent / "20261120-current-review" / "live-fixture-d295bc126897471ba7ddcc661216143b" / "src-tauri" / "target" / "debug" / "deps" / "github_issue_widget-fc25f1eb6ae85386.exe"
expected_hash = "193B97EDA5F965077229EE5F8D642A88BBB092BFC46470CA703FBFD3E1756EE1"
if hashlib.sha256(exe.read_bytes()).hexdigest().upper() != expected_hash:
    raise SystemExit("fixture hash mismatch")
test_name = "creation::tests::explicitly_approved_live_create_and_moves"
result = job_process.run([str(exe), "--list", "--exact", test_name], timeout=10, active_process_limit=1)
found = (test_name + ": test").encode() in result.stdout
print(f"exit={result.returncode}; expected_test_listed={found}")
if result.returncode != 0 or not found:
    raise SystemExit(1)
