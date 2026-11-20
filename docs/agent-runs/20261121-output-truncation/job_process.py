"""Bounded Windows process execution with a kill-on-close job.

Output is held in memory. Never use for unbounded-output programs.
"""
import ctypes
from ctypes import wintypes
import subprocess
import threading
import time

kernel = ctypes.WinDLL("kernel32", use_last_error=True)
ntdll = ctypes.WinDLL("ntdll")
kernel.CreateJobObjectW.argtypes = (ctypes.c_void_p, wintypes.LPCWSTR)
kernel.CreateJobObjectW.restype = wintypes.HANDLE
kernel.SetInformationJobObject.argtypes = (wintypes.HANDLE, ctypes.c_int, ctypes.c_void_p, wintypes.DWORD)
kernel.SetInformationJobObject.restype = wintypes.BOOL
kernel.AssignProcessToJobObject.argtypes = (wintypes.HANDLE, wintypes.HANDLE)
kernel.AssignProcessToJobObject.restype = wintypes.BOOL
kernel.TerminateJobObject.argtypes = (wintypes.HANDLE, wintypes.UINT)
kernel.TerminateJobObject.restype = wintypes.BOOL
kernel.QueryInformationJobObject.argtypes = (wintypes.HANDLE, ctypes.c_int, ctypes.c_void_p, wintypes.DWORD, ctypes.c_void_p)
kernel.QueryInformationJobObject.restype = wintypes.BOOL
kernel.CloseHandle.argtypes = (wintypes.HANDLE,)
kernel.CloseHandle.restype = wintypes.BOOL
ntdll.NtResumeProcess.argtypes = (wintypes.HANDLE,)
ntdll.NtResumeProcess.restype = ctypes.c_long


class BasicLimits(ctypes.Structure):
    _fields_ = [("PerProcessUserTimeLimit", ctypes.c_longlong), ("PerJobUserTimeLimit", ctypes.c_longlong),
                ("LimitFlags", wintypes.DWORD), ("MinimumWorkingSetSize", ctypes.c_size_t),
                ("MaximumWorkingSetSize", ctypes.c_size_t), ("ActiveProcessLimit", wintypes.DWORD),
                ("Affinity", ctypes.c_size_t), ("PriorityClass", wintypes.DWORD), ("SchedulingClass", wintypes.DWORD)]


class IoCounters(ctypes.Structure):
    _fields_ = [(name, ctypes.c_ulonglong) for name in ("ReadOperationCount", "WriteOperationCount", "OtherOperationCount", "ReadTransferCount", "WriteTransferCount", "OtherTransferCount")]


class ExtendedLimits(ctypes.Structure):
    _fields_ = [("BasicLimitInformation", BasicLimits), ("IoInfo", IoCounters),
                ("ProcessMemoryLimit", ctypes.c_size_t), ("JobMemoryLimit", ctypes.c_size_t),
                ("PeakProcessMemoryUsed", ctypes.c_size_t), ("PeakJobMemoryUsed", ctypes.c_size_t)]


class BasicAccounting(ctypes.Structure):
    _fields_ = [("TotalUserTime", ctypes.c_longlong), ("TotalKernelTime", ctypes.c_longlong),
                ("ThisPeriodTotalUserTime", ctypes.c_longlong), ("ThisPeriodTotalKernelTime", ctypes.c_longlong),
                ("TotalPageFaultCount", wintypes.DWORD), ("TotalProcesses", wintypes.DWORD),
                ("ActiveProcesses", wintypes.DWORD), ("TotalTerminatedProcesses", wintypes.DWORD)]


def wait_empty(job, timeout):
    deadline = time.monotonic() + timeout
    while True:
        info = BasicAccounting()
        if not kernel.QueryInformationJobObject(job, 1, ctypes.byref(info), ctypes.sizeof(info), None):
            raise RuntimeError("job accounting unavailable; outcome unknown")
        if info.ActiveProcesses == 0:
            return
        if time.monotonic() >= deadline:
            raise RuntimeError("job still has active processes; outcome unknown")
        time.sleep(min(0.05, max(0, deadline - time.monotonic())))


def run(args, *, timeout, cwd=None, env=None, cleanup_timeout=5, active_process_limit=None):
    job = kernel.CreateJobObjectW(None, None)
    if not job:
        raise ctypes.WinError(ctypes.get_last_error())
    process = None
    assigned = False
    try:
        limits = ExtendedLimits()
        limits.BasicLimitInformation.LimitFlags = 0x2000  # JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE
        if active_process_limit is not None:
            if active_process_limit != 1:
                raise ValueError("only a single-process limit is supported")
            limits.BasicLimitInformation.LimitFlags |= 0x00000008
            limits.BasicLimitInformation.ActiveProcessLimit = active_process_limit
        if not kernel.SetInformationJobObject(job, 9, ctypes.byref(limits), ctypes.sizeof(limits)):
            raise ctypes.WinError(ctypes.get_last_error())
        process = subprocess.Popen(args, cwd=cwd, env=env, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                                   stdin=subprocess.DEVNULL, creationflags=0x00000004 | subprocess.CREATE_NO_WINDOW)
        if not kernel.AssignProcessToJobObject(job, int(process._handle)):
            error = ctypes.get_last_error()
            process.kill()
            process.wait(timeout=cleanup_timeout)
            raise ctypes.WinError(error)
        assigned = True
        if ntdll.NtResumeProcess(int(process._handle)) != 0:
            raise RuntimeError("cannot resume process")
        state = {}
        def collect():
            try:
                state["output"] = process.communicate()
            except BaseException as error:
                state["error"] = error
        thread = threading.Thread(target=collect, daemon=True)
        thread.start()
        thread.join(timeout)
        if thread.is_alive():
            raise TimeoutError("process timed out; outcome unknown")
        if "error" in state:
            raise RuntimeError("output collection failed; outcome unknown") from state["error"]
        wait_empty(job, cleanup_timeout)
        return subprocess.CompletedProcess(args, process.returncode, *state["output"])
    finally:
        try:
            if assigned:
                if not kernel.TerminateJobObject(job, 1):
                    raise RuntimeError("job termination failed; outcome unknown")
                wait_empty(job, cleanup_timeout)
                if "thread" in locals():
                    thread.join(cleanup_timeout)
                    if thread.is_alive():
                        raise RuntimeError("pipe cleanup timed out; outcome unknown")
        finally:
            close_ok = kernel.CloseHandle(job)
            if process is not None:
                try:
                    process.wait(timeout=cleanup_timeout)
                except subprocess.TimeoutExpired as error:
                    raise RuntimeError("parent cleanup timed out; outcome unknown") from error
                finally:
                    if "thread" not in locals():
                        process.stdout.close()
                        process.stderr.close()
            if not close_ok:
                raise RuntimeError("job handle close failed; outcome unknown")
