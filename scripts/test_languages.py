#!/usr/bin/env python3
"""Multi-language test harness for LeetCode solutions."""

import json
import subprocess
import tempfile
import os
import sys
import argparse
from pathlib import Path

PROJECT_DIR = Path(__file__).parent.parent
SOLUTIONS_DIR = PROJECT_DIR / "scripts" / "test_solutions"
TESTCASES_DIR = PROJECT_DIR / "testcases"
PROBLEMS_DIR = PROJECT_DIR / "problems"

# Data structure helpers for Python
PYTHON_HELPERS = '''
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

def array_to_list(arr):
    if not arr:
        return None
    head = ListNode(arr[0])
    curr = head
    for val in arr[1:]:
        curr.next = ListNode(val)
        curr = curr.next
    return head

def list_to_array(head):
    result = []
    while head:
        result.append(head.val)
        head = head.next
    return result

def array_to_tree(arr):
    if not arr or arr[0] is None:
        return None
    root = TreeNode(arr[0])
    queue = [root]
    i = 1
    while queue and i < len(arr):
        node = queue.pop(0)
        if i < len(arr) and arr[i] is not None:
            node.left = TreeNode(arr[i])
            queue.append(node.left)
        i += 1
        if i < len(arr) and arr[i] is not None:
            node.right = TreeNode(arr[i])
            queue.append(node.right)
        i += 1
    return root

def tree_to_array(root):
    if not root:
        return []
    result = []
    queue = [root]
    while queue:
        node = queue.pop(0)
        if node:
            result.append(node.val)
            queue.append(node.left)
            queue.append(node.right)
        else:
            result.append(None)
    while result and result[-1] is None:
        result.pop()
    return result
'''

# Class-based problem handlers (function_name -> Python class name)
CLASS_FUNCS = {
    'LRUCache': 'LRUCache',
    'MinStack': 'MinStack',
    'trie': 'Trie',
    'wordDictionary': 'WordDictionary',
    'medianFinder': 'MedianFinder',
    'Twitter': 'Twitter',
    'KthLargest': 'KthLargest',
    'TimeMap': 'TimeMap',
    'DetectSquares': 'DetectSquares'
}

# Special encode/decode and serialize/deserialize function handlers
ROUNDTRIP_FUNCS = {'encodeDecode', 'codec'}

# Linked list input functions
LINKEDLIST_FUNCS = {
    'addTwoNumbers', 'removeNthFromEnd', 'mergeTwoLists',
    'reverseKGroup', 'reverseList', 'reorderList', 'hasCycle'
}

# Two tree input functions
TWO_TREE_FUNCS = {'isSameTree'}

# Tree input functions
TREE_FUNCS = {
    'invertTree', 'maxDepth', 'diameterOfBinaryTree', 'isBalanced',
    'isSubtree', 'levelOrder', 'rightSideView',
    'isValidBST', 'kthSmallest', 'maxPathSum', 'goodNodes',
    'lowestCommonAncestor'
}

# Tree output functions
TREE_OUTPUT_FUNCS = {'invertTree', 'buildTree'}

# Linked list output functions
LINKEDLIST_OUTPUT_FUNCS = {
    'addTwoNumbers', 'removeNthFromEnd', 'mergeTwoLists',
    'reverseKGroup', 'reverseList', 'reorderList'
}

# Order-independent comparison
ORDER_INDEPENDENT_FUNCS = {
    'subsets', 'subsetsWithDup', 'permute', 'permuteUnique',
    'combinationSum', 'combinationSum2', 'threeSum', 'letterCombinations',
    'generateParenthesis', 'partition', 'solveNQueens', 'groupAnagrams',
    'findWords', 'pacificAtlantic'
}

# In-place modification functions (result is the modified first argument)
IN_PLACE_FUNCS = {
    'rotate', 'setZeroes', 'solve', 'reorderList', 'wallsAndGates'
}

# Multiple valid answers (expected is list of acceptable answers)
MULTI_ANSWER_FUNCS = {'longestPalindrome'}

# Functions that need special handling for hasCycle (linked list with pos)
CYCLE_FUNCS = {'hasCycle'}

# Graph input functions
GRAPH_FUNCS = {'cloneGraph'}

# LCA needs special handling (p, q are values, not nodes)
LCA_FUNCS = {'lowestCommonAncestor'}

# Random pointer list functions
RANDOM_LIST_FUNCS = {'copyRandomList'}


def run_python_test(solution_file, func_name, test_cases, problem_info):
    """Run tests for a Python solution."""
    with open(solution_file) as f:
        solution_code = f.read()

    is_linkedlist = func_name in LINKEDLIST_FUNCS
    is_tree = func_name in TREE_FUNCS
    is_tree_output = func_name in TREE_OUTPUT_FUNCS
    is_linkedlist_output = func_name in LINKEDLIST_OUTPUT_FUNCS
    is_order_independent = func_name in ORDER_INDEPENDENT_FUNCS
    is_in_place = func_name in IN_PLACE_FUNCS
    is_multi_answer = func_name in MULTI_ANSWER_FUNCS
    is_cycle = func_name in CYCLE_FUNCS
    is_graph = func_name in GRAPH_FUNCS
    is_lca = func_name in LCA_FUNCS
    is_random_list = func_name in RANDOM_LIST_FUNCS
    is_two_tree = func_name in TWO_TREE_FUNCS

    test_code = f'''
import json
import sys
import copy

{PYTHON_HELPERS}

class Node:
    def __init__(self, val=0, neighbors=None):
        self.val = val
        self.neighbors = neighbors if neighbors else []

def array_to_graph(adj_list):
    if not adj_list:
        return None
    nodes = [Node(i + 1) for i in range(len(adj_list))]
    for i, neighbors in enumerate(adj_list):
        nodes[i].neighbors = [nodes[j - 1] for j in neighbors]
    return nodes[0] if nodes else None

def graph_to_array(node):
    if not node:
        return []
    visited = {{}}
    def dfs(n):
        if n.val in visited:
            return
        visited[n.val] = n
        for neighbor in n.neighbors:
            dfs(neighbor)
    dfs(node)
    result = [None] * len(visited)
    for val, n in visited.items():
        result[val - 1] = [neighbor.val for neighbor in n.neighbors]
    return result

def create_cycle_list(arr, pos):
    if not arr:
        return None
    head = ListNode(arr[0])
    curr = head
    cycle_node = head if pos == 0 else None
    for i, val in enumerate(arr[1:], 1):
        curr.next = ListNode(val)
        curr = curr.next
        if i == pos:
            cycle_node = curr
    if pos >= 0 and cycle_node:
        curr.next = cycle_node
    return head

def find_tree_node(root, val):
    if not root:
        return None
    if root.val == val:
        return root
    left = find_tree_node(root.left, val)
    if left:
        return left
    return find_tree_node(root.right, val)

class RandomNode:
    def __init__(self, val=0, next=None, random=None):
        self.val = val
        self.next = next
        self.random = random

def array_to_random_list(arr):
    if not arr:
        return None
    # Handle edge case of [[]] (list with empty element)
    arr = [x for x in arr if x and len(x) >= 2]
    if not arr:
        return None
    nodes = [RandomNode(x[0]) for x in arr]
    for i in range(len(nodes) - 1):
        nodes[i].next = nodes[i + 1]
    for i, x in enumerate(arr):
        if x[1] is not None and 0 <= x[1] < len(nodes):
            nodes[i].random = nodes[x[1]]
    return nodes[0]

def random_list_to_array(head):
    if not head:
        return []
    nodes = []
    node_to_idx = {{}}
    curr = head
    idx = 0
    while curr:
        nodes.append(curr)
        node_to_idx[id(curr)] = idx
        curr = curr.next
        idx += 1
    result = []
    for node in nodes:
        random_idx = node_to_idx[id(node.random)] if node.random else None
        result.append([node.val, random_idx])
    return result

{solution_code}

def normalize(val):
    if isinstance(val, list):
        return sorted([normalize(v) for v in val], key=lambda x: json.dumps(x, sort_keys=True, default=str))
    return val

def compare(expected, got, order_independent=False, multi_answer=False):
    if multi_answer and isinstance(expected, list):
        return got in expected
    if order_independent:
        return normalize(expected) == normalize(got)
    return expected == got

def float_compare(expected, got, tolerance=1e-5):
    if isinstance(expected, float) and isinstance(got, float):
        return abs(expected - got) < tolerance
    return expected == got

results = []
tests = json.loads('{json.dumps(test_cases).replace(chr(92), chr(92)+chr(92)).replace("'", chr(92)+"'")}')

for test in tests:
    inp = test.get('input', {{}})
    expected = test.get('expected')

    try:
        # Handle different input formats
        if isinstance(inp, dict):
            args = list(inp.values())
        else:
            args = list(inp) if isinstance(inp, list) else [inp]


        # Handle cycle linked list
        {'if len(args) >= 2 and isinstance(args[0], list): args = [create_cycle_list(args[0], args[1])]' if is_cycle else ''}

        # Convert linked list inputs
        {'args = [array_to_list(a) if isinstance(a, list) and len(a) > 0 and isinstance(a[0], (int, type(None))) else (array_to_list(a) if a == [] else a) for a in args]' if is_linkedlist else ''}

        # Convert graph inputs
        {'args = [array_to_graph(a) if isinstance(a, list) else a for a in args]' if is_graph else ''}

        # Convert tree inputs
        {'args = [array_to_tree(a) if isinstance(a, list) else a for a in args]' if is_tree else ''}

        # Convert two tree inputs (both args are trees)
        {'args = [array_to_tree(args[0]), array_to_tree(args[1] if len(args) > 1 else test.get(\"q\", []))]' if is_two_tree else ''}

        # Special handling for LCA - convert p, q values to TreeNode references
        {'if len(args) == 3: args = [args[0], find_tree_node(args[0], args[1]), find_tree_node(args[0], args[2])]' if is_lca else ''}

        # Convert random list inputs
        {'args = [array_to_random_list(a) if isinstance(a, list) and len(a) > 0 and isinstance(a[0], list) else a for a in args]' if is_random_list else ''}

        result = {func_name}(*args)

        # For in-place functions, get result from modified arg
        {'result = args[0] if result is None else result' if is_in_place else ''}

        # Convert linked list output
        {'result = list_to_array(result) if result is not None and hasattr(result, "val") and hasattr(result, "next") else ([] if result is None else result)' if is_linkedlist_output else ''}

        # Convert graph output
        {'result = graph_to_array(result) if result and hasattr(result, "neighbors") else ([] if result is None else result)' if is_graph else ''}

        # Convert tree output
        {'result = tree_to_array(result) if result and hasattr(result, "val") and hasattr(result, "left") else result' if is_tree_output else ''}

        # Convert LCA output to value
        {'result = result.val if result and hasattr(result, "val") else result' if is_lca else ''}

        # Convert random list output
        {'result = random_list_to_array(result) if result and hasattr(result, "random") else ([] if result is None else result)' if is_random_list else ''}

        # Float comparison for numeric results
        if isinstance(expected, float) and isinstance(result, float):
            passed = float_compare(expected, result)
        else:
            passed = compare(expected, result, {is_order_independent}, {is_multi_answer})
        results.append({{'input': test.get('input'), 'expected': expected, 'got': result, 'pass': passed}})
    except Exception as e:
        results.append({{'input': test.get('input'), 'expected': expected, 'got': None, 'error': str(e), 'pass': False}})

print(json.dumps(results))
'''

    with tempfile.NamedTemporaryFile(mode='w', suffix='.py', delete=False) as f:
        f.write(test_code)
        temp_file = f.name

    try:
        result = subprocess.run(
            ['python3', temp_file],
            capture_output=True,
            text=True,
            timeout=60
        )
        os.unlink(temp_file)

        if result.returncode == 0 and result.stdout.strip():
            return json.loads(result.stdout.strip())
        else:
            return [{'error': result.stderr or 'Unknown error', 'pass': False}]
    except subprocess.TimeoutExpired:
        os.unlink(temp_file)
        return [{'error': 'Timeout', 'pass': False}]
    except Exception as e:
        if os.path.exists(temp_file):
            os.unlink(temp_file)
        return [{'error': str(e), 'pass': False}]


def run_class_test_python(solution_file, class_name, test_cases):
    """Run tests for class-based Python solutions."""
    with open(solution_file) as f:
        solution_code = f.read()

    test_code = f'''
import json

{PYTHON_HELPERS}

{solution_code}

results = []
tests = json.loads('{json.dumps(test_cases).replace(chr(92), chr(92)+chr(92)).replace("'", chr(92)+"'")}')

for test in tests:
    inp = test.get('input', [])
    expected = test.get('expected')

    try:
        methods, args_list = inp[0], inp[1]

        # Instantiate class
        obj = {class_name}(*args_list[0])
        result_list = [None]

        # Call methods
        for i in range(1, len(methods)):
            method = getattr(obj, methods[i])
            res = method(*args_list[i])
            result_list.append(res)

        passed = result_list == expected
        results.append({{'input': inp, 'expected': expected, 'got': result_list, 'pass': passed}})
    except Exception as e:
        results.append({{'input': inp, 'expected': expected, 'got': None, 'error': str(e), 'pass': False}})

print(json.dumps(results))
'''

    with tempfile.NamedTemporaryFile(mode='w', suffix='.py', delete=False) as f:
        f.write(test_code)
        temp_file = f.name

    try:
        result = subprocess.run(
            ['python3', temp_file],
            capture_output=True,
            text=True,
            timeout=60
        )
        os.unlink(temp_file)

        if result.returncode == 0 and result.stdout.strip():
            return json.loads(result.stdout.strip())
        return [{'error': result.stderr or 'Unknown error', 'pass': False}]
    except Exception as e:
        if os.path.exists(temp_file):
            os.unlink(temp_file)
        return [{'error': str(e), 'pass': False}]


def run_roundtrip_test_python(solution_file, func_name, test_cases):
    """Run tests for encode/decode or serialize/deserialize Python solutions."""
    with open(solution_file) as f:
        solution_code = f.read()

    if func_name == 'encodeDecode':
        runner = 'result = decode(encode(test_input[0]))'
    else:  # codec
        runner = '''codec = Codec()
        tree = array_to_tree(test_input)
        serialized = codec.serialize(tree)
        deserialized = codec.deserialize(serialized)
        result = tree_to_array(deserialized)'''

    test_code = f'''
import json

{PYTHON_HELPERS}

{solution_code}

results = []
tests = json.loads('{json.dumps(test_cases).replace(chr(92), chr(92)+chr(92)).replace("'", chr(92)+"'")}')

for test in tests:
    inp = test.get('input', {{}})
    expected = test.get('expected')

    try:
        if isinstance(inp, dict):
            test_input = list(inp.values())
        else:
            test_input = inp[0] if isinstance(inp, list) and len(inp) == 1 else inp

        {runner}

        passed = result == expected
        results.append({{'input': inp, 'expected': expected, 'got': result, 'pass': passed}})
    except Exception as e:
        results.append({{'input': inp, 'expected': expected, 'got': None, 'error': str(e), 'pass': False}})

print(json.dumps(results))
'''

    with tempfile.NamedTemporaryFile(mode='w', suffix='.py', delete=False) as f:
        f.write(test_code)
        temp_file = f.name

    try:
        result = subprocess.run(
            ['python3', temp_file],
            capture_output=True,
            text=True,
            timeout=60
        )
        os.unlink(temp_file)

        if result.returncode == 0 and result.stdout.strip():
            return json.loads(result.stdout.strip())
        return [{'error': result.stderr or 'Unknown error', 'pass': False}]
    except Exception as e:
        if os.path.exists(temp_file):
            os.unlink(temp_file)
        return [{'error': str(e), 'pass': False}]


def test_problem_python(problem_id, solution_file, tc_file, problem_file):
    """Test a single Python problem."""
    with open(tc_file) as f:
        tc_data = json.load(f)

    with open(problem_file) as f:
        prob_info = json.load(f)

    func_name = prob_info.get('function_name', '')
    test_cases = tc_data.get('run_tests', []) + tc_data.get('submit_tests', [])

    if not test_cases:
        return None, 0, 0, "No test cases"

    # Check if class-based
    if func_name in CLASS_FUNCS:
        class_name = CLASS_FUNCS[func_name]
        results = run_class_test_python(solution_file, class_name, test_cases)
    elif func_name in ROUNDTRIP_FUNCS:
        results = run_roundtrip_test_python(solution_file, func_name, test_cases)
    else:
        results = run_python_test(solution_file, func_name, test_cases, prob_info)

    passed = sum(1 for r in results if r.get('pass'))
    total = len(results)
    failures = [r for r in results if not r.get('pass')]

    return failures[:3], passed, total, None


def main():
    parser = argparse.ArgumentParser(description='Test LeetCode solutions in multiple languages')
    parser.add_argument('--lang', choices=['python', 'cpp', 'c', 'all'], default='python',
                       help='Language to test')
    parser.add_argument('--problem', type=str, help='Test specific problem ID (e.g., 001)')
    args = parser.parse_args()

    if args.lang == 'python':
        ext = '.py'
    elif args.lang == 'cpp':
        ext = '.cpp'
    elif args.lang == 'c':
        ext = '.c'
    else:
        print("Testing all languages not yet implemented")
        return

    # Find solutions
    solution_files = sorted(SOLUTIONS_DIR.glob(f"*{ext}"))

    if args.problem:
        solution_files = [f for f in solution_files if f.stem.startswith(args.problem)]

    results = {
        'passed': [],
        'failed': [],
        'skipped': [],
        'total_tests': 0,
        'total_passed': 0
    }

    for sol_file in solution_files:
        problem_id = sol_file.stem.split('_')[0]

        # Find matching testcase and problem files
        tc_files = list(TESTCASES_DIR.glob(f"{problem_id}_*.json"))
        prob_files = list(PROBLEMS_DIR.glob(f"{problem_id}_*.json"))

        if not tc_files or not prob_files:
            results['skipped'].append((problem_id, "Missing files"))
            continue

        with open(prob_files[0]) as f:
            prob_info = json.load(f)
        func_name = prob_info.get('function_name', '')

        print(f"Testing {problem_id} ({func_name})...", end=' ', flush=True)

        if args.lang == 'python':
            failures, passed, total, error = test_problem_python(
                problem_id, sol_file, tc_files[0], prob_files[0]
            )
        else:
            failures, passed, total, error = None, 0, 0, "Not implemented"

        if error:
            print(f"SKIP ({error})")
            results['skipped'].append((problem_id, error))
        elif passed == total:
            print(f"PASS ({passed}/{total})")
            results['passed'].append(problem_id)
            results['total_tests'] += total
            results['total_passed'] += passed
        else:
            print(f"FAIL ({passed}/{total})")
            results['failed'].append((problem_id, func_name, failures))
            results['total_tests'] += total
            results['total_passed'] += passed

    # Summary
    print("\n" + "=" * 60)
    print("SUMMARY")
    print("=" * 60)
    print(f"Language: {args.lang.upper()}")
    print(f"Problems: {len(results['passed'])} passed, {len(results['failed'])} failed, {len(results['skipped'])} skipped")
    if results['total_tests'] > 0:
        pct = 100 * results['total_passed'] // results['total_tests']
        print(f"Test cases: {results['total_passed']}/{results['total_tests']} passed ({pct}%)")

    if results['failed']:
        print("\n" + "=" * 60)
        print("FAILURES")
        print("=" * 60)
        for problem_id, func_name, failures in results['failed']:
            print(f"\n{problem_id} ({func_name}):")
            for f in failures or []:
                if 'error' in f:
                    print(f"  ERROR: {f['error'][:80]}")
                else:
                    print(f"  in: {json.dumps(f.get('input'))[:60]}")
                    print(f"  exp: {f.get('expected')}, got: {f.get('got')}")


if __name__ == '__main__':
    main()
