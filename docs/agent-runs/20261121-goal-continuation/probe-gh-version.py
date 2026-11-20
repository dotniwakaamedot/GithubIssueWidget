"""Check gh startup under a single-process Job, without authentication or network."""
import hashlib
from pathlib import Path
import sys

sys.path.insert(0, str(Path(__file__).resolve().parent.parent / "20261121-output-truncation"))
import job_process

exe = Path(r"C:\Program Files\GitHub CLI\gh.exe")
digest = hashlib.sha256(exe.read_bytes()).hexdigest().upper()
result = job_process.run([str(exe), "--version"], timeout=10, active_process_limit=1)
recognized = result.stdout.startswith(b"gh version ")
print(f"binary_sha256={digest}; exit={result.returncode}; version_recognized={recognized}")
if result.returncode != 0 or not recognized:
    raise SystemExit(1)
