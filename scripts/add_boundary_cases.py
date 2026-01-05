#!/usr/bin/env python3
"""Add boundary and failure test cases to all problems."""

import json
import subprocess
import tempfile
import os
from pathlib import Path

PROJECT_DIR = Path(__file__).parent.parent
SOLUTIONS_DIR = PROJECT_DIR / "scripts" / "test_solutions"
TESTCASES_DIR = PROJECT_DIR / "testcases"
PROBLEMS_DIR = PROJECT_DIR / "problems"

# Problem-specific edge cases based on common LeetCode patterns
PROBLEM_SPECIFIC_CASES = {
    # Two Sum - no solution case, same element twice
    '001': [
        {'nums': [1, 2, 3], 'target': 10},  # no solution
        {'nums': [3, 3], 'target': 6},       # same element
        {'nums': [0, 0], 'target': 0},       # zeros
        {'nums': [-1, -2, -3], 'target': -5}, # negatives
    ],
    # Add Two Numbers - carry edge cases
    '002': [
        [[9, 9, 9], [1]],           # carry propagation
        [[0], [0]],                  # zeros
        [[9, 9, 9, 9], [9, 9, 9, 9]], # max carry
    ],
    # Longest Substring - repeating chars
    '003': [
        {'s': ''},                   # empty
        {'s': 'aaaaaaa'},           # all same
        {'s': 'abcabcabc'},         # repeating pattern
    ],
    # Valid Parentheses - unbalanced cases
    '020': [
        {'s': '('},                  # single open
        {'s': ')'},                  # single close
        {'s': '(((('},              # all open
        {'s': '))))'},              # all close
        {'s': '([)]'},              # interleaved wrong
        {'s': '((())'},             # unbalanced
    ],
    # Merge Two Sorted Lists
    '021': [
        [[], []],                    # both empty
        [[1], []],                   # one empty
        [[], [1]],                   # other empty
        [[1, 1, 1], [1, 1, 1]],     # all same
    ],
    # Search in Rotated Array - edge rotations
    '033': [
        {'nums': [1], 'target': 1},           # single found
        {'nums': [1], 'target': 0},           # single not found
        {'nums': [2, 1], 'target': 1},        # two elements
        {'nums': [1, 2, 3, 4, 5], 'target': 6}, # not rotated, not found
        {'nums': [3, 1], 'target': 3},        # pivot at start
    ],
    # Maximum Subarray
    '053': [
        {'nums': [-1]},                       # single negative
        {'nums': [-2, -1]},                   # all negative
        {'nums': [0]},                        # single zero
        {'nums': [-1, 0, -1]},               # zero in middle
    ],
    # Climbing Stairs - small cases
    '070': [
        {'n': 1},
        {'n': 2},
        {'n': 3},
        {'n': 45},  # larger case
    ],
    # Best Time to Buy and Sell Stock
    '121': [
        {'prices': [7, 6, 5, 4, 3, 2, 1]},   # always decreasing (no profit)
        {'prices': [1, 2, 3, 4, 5, 6, 7]},   # always increasing
        {'prices': [2, 1, 2, 1, 2, 1]},      # oscillating
        {'prices': [1]},                      # single price
        {'prices': [1, 1, 1, 1]},            # all same
    ],
    # Valid Palindrome
    '125': [
        {'s': ''},                            # empty
        {'s': 'a'},                           # single
        {'s': '.,'},                          # only punctuation
        {'s': '   '},                         # only spaces
        {'s': 'race a car'},                  # not palindrome
        {'s': '0P'},                          # alphanumeric
    ],
    # Linked List Cycle - cycle positions
    '141': [
        [[], -1],                             # empty
        [[1], -1],                            # single no cycle
        [[1], 0],                             # single with cycle to self
        [[1, 2], -1],                         # two no cycle
        [[1, 2], 0],                          # two cycle to first
        [[1, 2], 1],                          # two cycle to second
    ],
    # LRU Cache
    '146': [
        [['LRUCache', 'get'], [[1], [1]]],   # get from empty
        [['LRUCache', 'put', 'get'], [[1], [1, 1], [1]]], # single capacity
    ],
    # Number of Islands
    '200': [
        {'grid': [[]]},                       # empty grid
        {'grid': [['0']]},                    # single water
        {'grid': [['1']]},                    # single land
        {'grid': [['1', '1'], ['1', '1']]},  # all land
        {'grid': [['0', '0'], ['0', '0']]},  # all water
    ],
    # House Robber
    '198': [
        {'nums': []},                         # empty
        {'nums': [0]},                        # single zero
        {'nums': [100]},                      # single value
        {'nums': [1, 2]},                     # two houses
        {'nums': [2, 1]},                     # two houses reversed
    ],
    # Binary Search
    '704': [
        {'nums': [], 'target': 1},            # empty array
        {'nums': [1], 'target': 1},           # single found
        {'nums': [1], 'target': 0},           # single not found
        {'nums': [1, 2], 'target': 1},        # two elements first
        {'nums': [1, 2], 'target': 2},        # two elements second
        {'nums': [1, 2], 'target': 0},        # two elements not found
    ],
}


def compute_expected(solution_file, func_name, input_case):
    """Run solution to compute expected value."""
    with open(solution_file) as f:
        solution = f.read()

    helpers = '''
const arrayToTree = (arr) => {
    if (!arr || arr.length === 0 || arr[0] === null) return null;
    const root = { val: arr[0], left: null, right: null };
    const queue = [root];
    let i = 1;
    while (queue.length > 0 && i < arr.length) {
        const node = queue.shift();
        if (i < arr.length && arr[i] !== null) {
            node.left = { val: arr[i], left: null, right: null };
            queue.push(node.left);
        }
        i++;
        if (i < arr.length && arr[i] !== null) {
            node.right = { val: arr[i], left: null, right: null };
            queue.push(node.right);
        }
        i++;
    }
    return root;
};
const treeToArray = (root) => {
    if (!root) return [];
    const result = [];
    const queue = [root];
    while (queue.length > 0) {
        const node = queue.shift();
        if (node) { result.push(node.val); queue.push(node.left); queue.push(node.right); }
        else { result.push(null); }
    }
    while (result.length > 0 && result[result.length - 1] === null) result.pop();
    return result;
};
const arrayToList = (arr) => {
    if (!arr || arr.length === 0) return null;
    let head = { val: arr[0], next: null };
    let curr = head;
    for (let i = 1; i < arr.length; i++) { curr.next = { val: arr[i], next: null }; curr = curr.next; }
    return head;
};
const listToArray = (head) => {
    const result = [];
    while (head) { result.push(head.val); head = head.next; }
    return result;
};
'''

    # Convert input for the test
    if isinstance(input_case, dict):
        inputs = list(input_case.values())
    else:
        inputs = input_case

    test_code = solution + helpers + f'''
try {{
    let inputs = {json.dumps(inputs)};
    let result = {func_name}(...inputs);
    if (result && typeof result === 'object' && 'val' in result && 'next' in result) result = listToArray(result);
    if (result && typeof result === 'object' && 'val' in result && 'left' in result) result = treeToArray(result);
    console.log(JSON.stringify({{ success: true, result }}));
}} catch (e) {{
    console.log(JSON.stringify({{ success: false, error: e.message }}));
}}
'''

    with tempfile.NamedTemporaryFile(mode='w', suffix='.js', delete=False) as f:
        f.write(test_code)
        temp_file = f.name

    try:
        result = subprocess.run(['bun', 'run', temp_file], capture_output=True, text=True, timeout=5)
        os.unlink(temp_file)
        if result.returncode == 0 and result.stdout.strip():
            data = json.loads(result.stdout.strip())
            if data.get('success'):
                return data.get('result')
        return None
    except:
        if os.path.exists(temp_file):
            os.unlink(temp_file)
        return None


def add_specific_cases(problem_id):
    """Add problem-specific edge cases."""
    if problem_id not in PROBLEM_SPECIFIC_CASES:
        return 0

    # Get files
    prob_files = list(PROBLEMS_DIR.glob(f"{problem_id}_*.json"))
    sol_files = list(SOLUTIONS_DIR.glob(f"{problem_id}_*.js"))
    tc_files = list(TESTCASES_DIR.glob(f"{problem_id}_*.json"))

    if not all([prob_files, sol_files, tc_files]):
        return 0

    with open(prob_files[0]) as f:
        prob_info = json.load(f)
    func_name = prob_info.get('function_name', '')

    with open(tc_files[0]) as f:
        tc_data = json.load(f)

    existing = tc_data.get('run_tests', []) + tc_data.get('submit_tests', [])
    existing_inputs = {json.dumps(t.get('input'), sort_keys=True) for t in existing}

    added = 0
    for case in PROBLEM_SPECIFIC_CASES[problem_id]:
        key = json.dumps(case, sort_keys=True)
        if key in existing_inputs:
            continue

        expected = compute_expected(sol_files[0], func_name, case)
        if expected is not None:
            tc_data.setdefault('submit_tests', []).append({'input': case, 'expected': expected})
            existing_inputs.add(key)
            added += 1

    if added > 0:
        with open(tc_files[0], 'w') as f:
            json.dump(tc_data, f, indent=2)

    return added


def main():
    total = 0
    updated = 0

    for prob_id in PROBLEM_SPECIFIC_CASES:
        print(f"Processing {prob_id}...", end=' ', flush=True)
        added = add_specific_cases(prob_id)
        if added > 0:
            print(f"Added {added} cases")
            total += added
            updated += 1
        else:
            print("No new cases")

    print(f"\n{'='*60}")
    print(f"Added {total} boundary/failure cases to {updated} problems")


if __name__ == '__main__':
    main()
