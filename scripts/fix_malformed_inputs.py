#!/usr/bin/env python3
"""Fix malformed test case inputs - convert object format to array format."""

import json
from pathlib import Path

TESTCASES_DIR = Path(__file__).parent.parent / "testcases"

def fix_testcase_file(filepath: Path) -> tuple[int, int]:
    """Fix malformed inputs in a testcase file. Returns (fixed_count, total_tests)."""
    with open(filepath) as f:
        data = json.load(f)

    fixed_count = 0

    # Fix run_tests
    for tc in data.get('run_tests', []):
        if isinstance(tc.get('input'), dict):
            tc['input'] = list(tc['input'].values())
            fixed_count += 1

    # Fix submit_tests
    for tc in data.get('submit_tests', []):
        if isinstance(tc.get('input'), dict):
            tc['input'] = list(tc['input'].values())
            fixed_count += 1

    total = len(data.get('run_tests', [])) + len(data.get('submit_tests', []))

    if fixed_count > 0:
        with open(filepath, 'w') as f:
            json.dump(data, f, indent=2)
        print(f"  Fixed {fixed_count}/{total} tests in {filepath.name}")

    return fixed_count, total

def main():
    total_fixed = 0
    files_fixed = 0

    for filepath in sorted(TESTCASES_DIR.glob("*.json")):
        fixed, total = fix_testcase_file(filepath)
        if fixed > 0:
            total_fixed += fixed
            files_fixed += 1

    print(f"\nTotal: Fixed {total_fixed} malformed inputs across {files_fixed} files")

if __name__ == "__main__":
    main()
