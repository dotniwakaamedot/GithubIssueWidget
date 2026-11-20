import importlib.util
from pathlib import Path
import unittest
import sys
import job_process

spec = importlib.util.spec_from_file_location("run_live", Path(__file__).with_name("run-live.py"))
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)


class ResultTests(unittest.TestCase):
    def test_child_output_through_result_checker(self):
        complete = "LIVE_KIND=created LIVE_ISSUE=42\nLIVE_STEP=in-progress\nLIVE_STEP=todo\nLIVE_STEP=done\nLIVE_STEP=in-progress\nLIVE_STEP=done\nLIVE_STEP=todo\nLIVE_STEP=done\n"
        for output, code, expected in [(complete, 0, True), ("running 0 tests\n", 0, False), (complete.splitlines()[0], 1, False)]:
            child = job_process.run([sys.executable, "-c", "import sys; sys.stdout.write(sys.argv[1]); sys.exit(int(sys.argv[2]))", output, str(code)], timeout=5)
            self.assertEqual(runner.checked_results(child.stdout, child.returncode)[1], expected)

    def test_requires_exact_success_sequence_and_exit(self):
        complete = b"test name ... LIVE_KIND=created LIVE_ISSUE=42\nLIVE_STEP=in-progress\nLIVE_STEP=todo\nLIVE_STEP=done\nLIVE_STEP=in-progress\nLIVE_STEP=done\nLIVE_STEP=todo\nLIVE_STEP=done\n"
        self.assertTrue(runner.checked_results(complete, 0)[1])
        for output, code in [(b"running 0 tests\n", 0), (complete, 1), (complete.rsplit(b"LIVE_STEP=done", 1)[0], 0), (complete.replace(b"ISSUE=42", b"ISSUE=0"), 0)]:
            self.assertFalse(runner.checked_results(output, code)[1])


if __name__ == "__main__":
    unittest.main()
