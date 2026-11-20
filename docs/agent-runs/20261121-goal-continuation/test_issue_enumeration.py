import importlib.util
import io
from contextlib import redirect_stderr
from pathlib import Path
import unittest
from unittest.mock import patch

spec = importlib.util.spec_from_file_location("issue_enumeration", Path(__file__).with_name("issue-enumeration.py"))
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)


def issue(number, body=None):
    return {"number": number, "title": "synthetic", "body": body}


class EnumerationTests(unittest.TestCase):
    def test_empty_and_multiple_items_are_counted(self):
        self.assertEqual(module.collect(lambda _: ([], None)), (1, 0, []))
        self.assertEqual(module.collect(lambda _: ([issue(1), issue(2, module.MARKER)], None)), (1, 2, [2]))

    def test_exact_full_page_requires_a_following_empty_page(self):
        calls = []
        def fetch(page):
            calls.append(page)
            return ([issue(i) for i in range(100)], 2) if page == 1 else ([], None)
        self.assertEqual(module.collect(fetch), (2, 100, []))
        self.assertEqual(calls, [1, 2])

    def test_full_page_followed_by_match(self):
        def fetch(page):
            return ([issue(i) for i in range(100)], 2) if page == 1 else ([issue(101, module.MARKER)], None)
        self.assertEqual(module.collect(fetch), (2, 101, [101]))

    def test_invalid_page_or_issue_is_not_success(self):
        with self.assertRaises(ValueError):
            module.collect(lambda _: ({"items": []}, None))
        with self.assertRaises(ValueError):
            module.collect(lambda _: ([{"number": 1, "title": None}], None))
        with self.assertRaises(ValueError):
            module.collect(lambda _: ([{"number": 1, "title": "x"}], None))

    def test_short_page_with_next_is_followed(self):
        self.assertEqual(module.collect(lambda p: ([issue(p)], 2) if p == 1 else ([issue(2, module.MARKER)], None)), (2, 2, [2]))

    def test_next_link_and_end_are_parsed(self):
        link = b'<https://api.github.com/repos/dotniwakaamedot/GithubIssueWidget/issues?state=all&per_page=100&page=2>; rel="next"'
        page, next_page = module.parse_response(b'HTTP/2.0 200 OK\r\nLink: ' + link + b'\r\n\r\n[]', 1)
        self.assertEqual((page, next_page), ([], 2))
        self.assertEqual(module.parse_response(b'HTTP/2.0 200 OK\nX-Test: value\n\n[]', 1), ([], None))

    def test_invalid_or_looping_next_is_rejected(self):
        with self.assertRaises(ValueError):
            module.collect(lambda _: ([], 1))
        with self.assertRaises(ValueError):
            module.parse_response(b'HTTP/2.0 200 OK\nLink: <https://evil.example/issues?page=2>; rel="next"\n\n[]', 1)
        with self.assertRaises(ValueError):
            module.parse_response(b'HTTP/2.0 200 OK\nLink: invalid\n\n[]', 1)

    def test_later_page_failure_is_not_success(self):
        def fetch(page):
            if page == 1:
                return ([issue(1)], 2)
            raise ValueError("synthetic secret should not be printed")
        with self.assertRaises(ValueError):
            module.collect(fetch)
        output = io.StringIO()
        with patch.object(module, "fetch_page", side_effect=ValueError("synthetic secret")), redirect_stderr(output):
            self.assertEqual(module.main(), 1)
        self.assertNotIn("synthetic secret", output.getvalue())


if __name__ == "__main__":
    unittest.main()
