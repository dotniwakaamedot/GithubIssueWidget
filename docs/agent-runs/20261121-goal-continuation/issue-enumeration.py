"""Read public repository issue pages; print only counts and matching numbers."""
import json
import re
import subprocess
import sys
from urllib.parse import parse_qs, urlsplit

MARKER = "giw-live-d295bc126897471ba7ddcc661216143b"
REPO = "dotniwakaamedot/GithubIssueWidget"


def collect(fetch):
    total = 0
    matches = []
    for page in range(1, 101):
        issues, next_page = fetch(page)
        if not isinstance(issues, list) or len(issues) > 100:
            raise ValueError("invalid page")
        for issue in issues:
            if not isinstance(issue, dict) or not isinstance(issue.get("number"), int):
                raise ValueError("invalid issue")
            if "title" not in issue or "body" not in issue:
                raise ValueError("missing issue fields")
            title, body = issue["title"], issue["body"]
            if not isinstance(title, str) or not (body is None or isinstance(body, str)):
                raise ValueError("invalid issue fields")
            if MARKER in title or (body is not None and MARKER in body):
                matches.append(issue["number"])
        total += len(issues)
        if next_page is None:
            return page, total, matches
        if next_page != page + 1:
            raise ValueError("invalid next page")
    raise ValueError("page limit reached")


class CliError(Exception):
    def __init__(self, code):
        self.code = code


def parse_response(raw, page):
    header, separator, body = raw.partition(b"\r\n\r\n")
    if not separator:
        header, separator, body = raw.partition(b"\n\n")
    if not separator:
        raise ValueError("missing headers")
    lines = header.decode("iso-8859-1").splitlines()
    if not lines or not re.fullmatch(r"HTTP/\S+ 200(?: .*)?", lines[0]):
        raise ValueError("unexpected HTTP status")
    links = []
    for line in lines[1:]:
        name, colon, value = line.partition(":")
        if not colon or not re.fullmatch(r"[!#$%&'*+.^_`|~0-9A-Za-z-]+", name):
            raise ValueError("invalid header")
        if name.lower() == "link":
            links.extend(value.strip().split(","))
    next_page = None
    for link in links:
        match = re.fullmatch(r'\s*<([^<>]+)>\s*;\s*rel="([^"]+)"\s*', link)
        if not match:
            raise ValueError("invalid link")
        relations = match.group(2).split()
        if not relations or any(not re.fullmatch(r"[A-Za-z][A-Za-z0-9.-]*", relation) for relation in relations):
            raise ValueError("invalid link relation")
        relations = [relation.lower() for relation in relations]
        if relations.count("next") > 1:
            raise ValueError("duplicate next")
        if "next" not in relations:
            continue
        if next_page is not None:
            raise ValueError("duplicate next")
        url = urlsplit(match.group(1))
        query = parse_qs(url.query, keep_blank_values=True)
        if (url.scheme != "https" or url.netloc.lower() != "api.github.com"
                or url.path.lower() != f"/repos/{REPO}/issues".lower()
                or url.fragment or query != {"state": ["all"], "per_page": ["100"], "page": [str(page + 1)]}):
            raise ValueError("invalid next URL")
        next_page = page + 1
    return json.loads(body), next_page


def fetch_page(page):
    path = f"repos/{REPO}/issues?state=all&per_page=100&page={page}"
    result = subprocess.run(["gh", "api", "--include", path], capture_output=True, timeout=30, check=False)
    if result.returncode:
        raise CliError(result.returncode)
    return parse_response(result.stdout, page)


def main():
    try:
        pages, total, matches = collect(fetch_page)
    except CliError as error:
        print(f"issue enumeration failed: cli exit={error.code}", file=sys.stderr)
        return 1
    except subprocess.TimeoutExpired:
        print("issue enumeration failed: timeout", file=sys.stderr)
        return 1
    except OSError:
        print("issue enumeration failed: process unavailable", file=sys.stderr)
        return 1
    except ValueError:
        print("issue enumeration failed: invalid response", file=sys.stderr)
        return 1
    print(f"pages={pages} issues_and_prs={total} marker_matches={len(matches)}")
    for number in matches:
        print(f"marker_issue={number}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
