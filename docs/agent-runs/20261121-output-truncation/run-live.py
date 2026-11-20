"""One-shot live check. Review and pin hashes before invoking this script."""
import hashlib
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import job_process

ROOT = Path(__file__).resolve().parents[1] / "20261120-current-review" / "live-fixture-d295bc126897471ba7ddcc661216143b" / "src-tauri"
EXE = ROOT / "target" / "debug" / "deps" / "github_issue_widget-fc25f1eb6ae85386.exe"
SOURCE_SHA = "15B29DE5A997FBF93EB8A2D038122C87256FC615944118A2D688D3D68D0B6E5F"
EXE_SHA = "193B97EDA5F965077229EE5F8D642A88BBB092BFC46470CA703FBFD3E1756EE1"


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest().upper()


def checked_results(stdout, returncode):
    events = []
    for line in stdout.decode("utf-8", errors="replace").splitlines():
        match = re.search(r"(?:^| )(?P<result>LIVE_KIND=(?:created|partial|uncertain) LIVE_ISSUE=[0-9]+|LIVE_STEP=(?:todo|in-progress|done))$", line)
        if match:
            events.append(match.group("result"))
    expected_steps = ["in-progress", "todo", "done", "in-progress", "done", "todo", "done"]
    success = bool(events and re.fullmatch(r"LIVE_KIND=created LIVE_ISSUE=[1-9][0-9]*", events[0]))
    success = success and events[1:] == [f"LIVE_STEP={step}" for step in expected_steps] and returncode == 0
    return events, success


def main():
    print("execution disabled: process-tree timeout handling requires review")
    return 1
    if digest(ROOT / "src" / "creation.rs") != SOURCE_SHA or digest(EXE) != EXE_SHA:
        print("source or executable hash mismatch")
        return 1
    try:
        gh = shutil.which("gh")
        if not gh:
            print("authentication unavailable")
            return 1
        auth = job_process.run([gh, "auth", "token"], timeout=20)
        if auth.returncode != 0:
            print("authentication unavailable")
            return 1
        token = auth.stdout.decode("utf-8").strip()
        if not token or "\n" in token or "\r" in token:
            print("authentication unavailable")
            return 1
        env = os.environ.copy()
        env["GIW_LIVE_TOKEN"] = token
        result = job_process.run([str(EXE), "--exact", "creation::tests::explicitly_approved_live_create_and_moves", "--ignored", "--nocapture", "--test-threads=1"], cwd=ROOT, env=env, timeout=600)
    except (OSError, subprocess.SubprocessError, UnicodeError, RuntimeError, TimeoutError):
        print("execution failed or timed out; inspect marker before any retry")
        return 1
    finally:
        if "env" in locals():
            env.pop("GIW_LIVE_TOKEN", None)
    events, success = checked_results(result.stdout, result.returncode)
    for event in events:
        print(event)
    if not success:
        print("live check failed; inspect marker before any retry")
        return 1
    print("live check completed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
