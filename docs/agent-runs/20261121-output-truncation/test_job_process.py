import subprocess
import sys
import time
import unittest
import ctypes
from pathlib import Path
import tempfile
import threading
from unittest import mock

import job_process


class JobTests(unittest.TestCase):
    def test_single_process_job_blocks_child_execution(self):
        script = """
import json, subprocess, sys
results = []
for flag in (0, subprocess.CREATE_BREAKAWAY_FROM_JOB):
    try:
        child = subprocess.run([sys.executable, '-c', "print('CHILD_EXECUTED')"],
                               stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                               creationflags=flag, timeout=3)
        results.append('executed' if b'CHILD_EXECUTED' in child.stdout else 'not-executed')
    except (OSError, subprocess.TimeoutExpired):
        results.append('not-executed')
print(json.dumps(results))
"""
        result = job_process.run([sys.executable, "-c", script], timeout=10, active_process_limit=1)
        self.assertEqual(result.returncode, 0, result.stderr.decode("utf-8", errors="replace"))
        self.assertEqual(result.stdout.strip(), b'["not-executed", "not-executed"]')

    def watch_process(self, pidfile):
        kernel = ctypes.WinDLL("kernel32", use_last_error=True)
        kernel.OpenProcess.argtypes = (ctypes.c_ulong, ctypes.c_int, ctypes.c_ulong)
        kernel.OpenProcess.restype = ctypes.c_void_p
        kernel.WaitForSingleObject.argtypes = (ctypes.c_void_p, ctypes.c_ulong)
        kernel.WaitForSingleObject.restype = ctypes.c_ulong
        kernel.CloseHandle.argtypes = (ctypes.c_void_p,)
        kernel.CloseHandle.restype = ctypes.c_int
        result = {}
        def observe():
            deadline = time.monotonic() + 3
            while time.monotonic() < deadline:
                try:
                    pid = int(pidfile.read_text())
                    handle = kernel.OpenProcess(0x00100000, False, pid)
                    if handle:
                        if kernel.WaitForSingleObject(handle, 0) == 258:
                            result["handle"] = handle
                            return
                        kernel.CloseHandle(handle)
                except (FileNotFoundError, ValueError):
                    pass
                time.sleep(0.01)
            result["error"] = "could not acquire descendant handle while running"
        watcher = threading.Thread(target=observe)
        watcher.start()
        def assert_exited():
            watcher.join(4)
            self.assertFalse(watcher.is_alive(), "descendant observation timed out")
            self.assertIn("handle", result, result.get("error"))
            try:
                self.assertEqual(kernel.WaitForSingleObject(result["handle"], 3000), 0)
            finally:
                kernel.CloseHandle(result["handle"])
        return assert_exited

    def test_success_and_captured_output(self):
        result = job_process.run([sys.executable, "-c", "print('synthetic')"], timeout=5)
        self.assertEqual(result.returncode, 0)
        self.assertEqual(result.stdout.strip(), b"synthetic")

    def test_timeout_is_bounded(self):
        start = time.monotonic()
        with self.assertRaises(TimeoutError):
            job_process.run([sys.executable, "-c", "import time; time.sleep(30)"], timeout=0.4, cleanup_timeout=3)
        self.assertLess(time.monotonic() - start, 6)

    def test_descendant_holding_pipe_is_terminated(self):
        with tempfile.TemporaryDirectory() as directory:
            pidfile = Path(directory) / "pid"
            child = "import os,sys,time; open(sys.argv[1],'w').write(str(os.getpid())); time.sleep(30)"
            code = "import subprocess,sys; subprocess.Popen([sys.executable,'-c',sys.argv[1],sys.argv[2]]); print('parent finished')"
            assert_exited = self.watch_process(pidfile)
            start = time.monotonic()
            with self.assertRaises(TimeoutError):
                job_process.run([sys.executable, "-c", code, child, str(pidfile)], timeout=2, cleanup_timeout=3)
            self.assertLess(time.monotonic() - start, 7)
            assert_exited()

    def test_descendant_with_closed_pipes_is_terminated(self):
        with tempfile.TemporaryDirectory() as directory:
            pidfile = Path(directory) / "pid"
            child = "import os,sys,time; os.close(1); os.close(2); open(sys.argv[1],'w').write(str(os.getpid())); time.sleep(30)"
            parent = "import subprocess,sys,time; subprocess.Popen([sys.executable,'-c',sys.argv[1],sys.argv[2]], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL); time.sleep(0.5)"
            assert_exited = self.watch_process(pidfile)
            with self.assertRaises(RuntimeError):
                job_process.run([sys.executable, "-c", parent, child, str(pidfile)], timeout=5, cleanup_timeout=1)
            assert_exited()

    def test_accounting_failure_is_not_success(self):
        with mock.patch.object(job_process.kernel, "QueryInformationJobObject", return_value=0):
            with self.assertRaisesRegex(RuntimeError, "outcome unknown"):
                job_process.run([sys.executable, "-c", "pass"], timeout=5)

    def test_assignment_and_resume_failure_are_not_success(self):
        with mock.patch.object(job_process.kernel, "AssignProcessToJobObject", return_value=0):
            with self.assertRaises(OSError):
                job_process.run([sys.executable, "-c", "pass"], timeout=5)
        with mock.patch.object(job_process.ntdll, "NtResumeProcess", return_value=-1):
            with self.assertRaisesRegex(RuntimeError, "resume"):
                job_process.run([sys.executable, "-c", "pass"], timeout=5)

    def test_termination_failure_is_not_success(self):
        with mock.patch.object(job_process.kernel, "TerminateJobObject", return_value=0):
            with self.assertRaisesRegex(RuntimeError, "outcome unknown"):
                job_process.run([sys.executable, "-c", "pass"], timeout=5)

    def test_failure_paths_with_live_descendant(self):
        with tempfile.TemporaryDirectory() as directory:
            for failure, holds_pipe in ((f, h) for f in ("query", "terminate", "close") for h in (False, True)):
                with self.subTest(failure=failure, holds_pipe=holds_pipe):
                    pidfile = Path(directory) / (failure + str(holds_pipe))
                    child = "import os,sys,time; open(sys.argv[1],'w').write(str(os.getpid())); time.sleep(30)"
                    redirects = "{}" if holds_pipe else "{'stdout':subprocess.DEVNULL,'stderr':subprocess.DEVNULL}"
                    parent = "import subprocess,sys,time; subprocess.Popen([sys.executable,'-c',sys.argv[1],sys.argv[2]], **" + redirects + "); time.sleep(0.5)"
                    assert_exited = self.watch_process(pidfile)
                    kernel = job_process.kernel
                    real_close = kernel.CloseHandle
                    if failure == "query":
                        patcher = mock.patch.object(kernel, "QueryInformationJobObject", return_value=0)
                    elif failure == "terminate":
                        patcher = mock.patch.object(kernel, "TerminateJobObject", return_value=0)
                    else:
                        def close_then_fail(handle):
                            real_close(handle)
                            return 0
                        patcher = mock.patch.object(kernel, "CloseHandle", side_effect=close_then_fail)
                    with patcher:
                        with self.assertRaisesRegex(RuntimeError, "outcome unknown"):
                            job_process.run([sys.executable, "-c", parent, child, str(pidfile)], timeout=2, cleanup_timeout=1)
                    assert_exited()


if __name__ == "__main__":
    unittest.main()
