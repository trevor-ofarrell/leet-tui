#!/usr/bin/env python3
"""Unified parallel test runner for LeetCode solutions in multiple languages."""

import argparse
import json
import os
import subprocess
import sys
import tempfile
import time
from concurrent.futures import ProcessPoolExecutor, as_completed
from dataclasses import dataclass, field
from pathlib import Path
from typing import Optional

# Check for rich library
try:
    from rich.console import Console
    from rich.live import Live
    from rich.panel import Panel
    from rich.progress import Progress, SpinnerColumn, BarColumn, TextColumn, TaskID
    from rich.table import Table
    from rich.text import Text
    RICH_AVAILABLE = True
except ImportError:
    RICH_AVAILABLE = False

PROJECT_DIR = Path(__file__).parent.parent
SOLUTIONS_DIR = PROJECT_DIR / "scripts" / "test_solutions"
TESTCASES_DIR = PROJECT_DIR / "testcases"
PROBLEMS_DIR = PROJECT_DIR / "problems"


@dataclass
class TestResult:
    """Result of testing a single problem/language combination."""
    problem_id: str
    language: str
    passed: int = 0
    total: int = 0
    failures: list = field(default_factory=list)
    error: Optional[str] = None
    duration_ms: float = 0.0

    @property
    def success(self) -> bool:
        return self.error is None and self.passed == self.total and self.total > 0


def get_problem_info(problem_id: str) -> Optional[dict]:
    """Load problem metadata from problems directory."""
    prob_files = list(PROBLEMS_DIR.glob(f"{problem_id}_*.json"))
    if not prob_files:
        return None
    with open(prob_files[0]) as f:
        return json.load(f)


def get_test_cases(problem_id: str) -> Optional[list]:
    """Load test cases for a problem."""
    tc_files = list(TESTCASES_DIR.glob(f"{problem_id}_*.json"))
    if not tc_files:
        return None
    with open(tc_files[0]) as f:
        data = json.load(f)
    return data.get('run_tests', []) + data.get('submit_tests', [])


# ============================================================================
# Language Runner Base
# ============================================================================

def run_subprocess(cmd: list, code: str, suffix: str, timeout: int = 60) -> tuple:
    """Execute code via subprocess and return (stdout, stderr, returncode)."""
    with tempfile.NamedTemporaryFile(mode='w', suffix=suffix, delete=False) as f:
        f.write(code)
        temp_file = f.name

    try:
        result = subprocess.run(
            cmd + [temp_file],
            capture_output=True,
            text=True,
            timeout=timeout
        )
        return result.stdout, result.stderr, result.returncode
    except subprocess.TimeoutExpired:
        return "", "Timeout", -1
    finally:
        if os.path.exists(temp_file):
            os.unlink(temp_file)


# ============================================================================
# Python Runner
# ============================================================================

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

class Node:
    def __init__(self, val=0, neighbors=None):
        self.val = val
        self.neighbors = neighbors if neighbors else []

class RandomNode:
    def __init__(self, val=0, next=None, random=None):
        self.val = val
        self.next = next
        self.random = random

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
    visited = {}
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

def array_to_random_list(arr):
    if not arr:
        return None
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
    node_to_idx = {}
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
'''

# Function categories for Python
CLASS_FUNCS = {
    'LRUCache': 'LRUCache', 'MinStack': 'MinStack', 'trie': 'Trie',
    'wordDictionary': 'WordDictionary', 'medianFinder': 'MedianFinder',
    'Twitter': 'Twitter', 'KthLargest': 'KthLargest', 'TimeMap': 'TimeMap',
    'DetectSquares': 'DetectSquares'
}
ROUNDTRIP_FUNCS = {'encodeDecode', 'codec'}
LINKEDLIST_FUNCS = {'addTwoNumbers', 'removeNthFromEnd', 'mergeTwoLists', 'reverseKGroup', 'reverseList', 'reorderList', 'hasCycle'}
TWO_TREE_FUNCS = {'isSameTree'}
TREE_FUNCS = {'invertTree', 'maxDepth', 'diameterOfBinaryTree', 'isBalanced', 'isSubtree', 'levelOrder', 'rightSideView', 'isValidBST', 'kthSmallest', 'maxPathSum', 'goodNodes', 'lowestCommonAncestor'}
TREE_OUTPUT_FUNCS = {'invertTree', 'buildTree'}
LINKEDLIST_OUTPUT_FUNCS = {'addTwoNumbers', 'removeNthFromEnd', 'mergeTwoLists', 'reverseKGroup', 'reverseList', 'reorderList'}
ORDER_INDEPENDENT_FUNCS = {'subsets', 'subsetsWithDup', 'permute', 'permuteUnique', 'combinationSum', 'combinationSum2', 'threeSum', 'letterCombinations', 'generateParenthesis', 'partition', 'solveNQueens', 'groupAnagrams', 'findWords', 'pacificAtlantic'}
IN_PLACE_FUNCS = {'rotate', 'setZeroes', 'solve', 'reorderList', 'wallsAndGates'}
MULTI_ANSWER_FUNCS = {'longestPalindrome'}
CYCLE_FUNCS = {'hasCycle'}
GRAPH_FUNCS = {'cloneGraph'}
LCA_FUNCS = {'lowestCommonAncestor'}
RANDOM_LIST_FUNCS = {'copyRandomList'}


def run_python_test(solution_file: Path, func_name: str, test_cases: list) -> list:
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

{solution_code}

results = []
tests = json.loads('{json.dumps(test_cases).replace(chr(92), chr(92)+chr(92)).replace("'", chr(92)+"'")}')

for test in tests:
    inp = test.get('input', {{}})
    expected = test.get('expected')

    try:
        if isinstance(inp, dict):
            args = list(inp.values())
        else:
            args = list(inp) if isinstance(inp, list) else [inp]

        {'if len(args) >= 2 and isinstance(args[0], list): args = [create_cycle_list(args[0], args[1])]' if is_cycle else ''}
        {'args = [array_to_list(a) if isinstance(a, list) and len(a) > 0 and isinstance(a[0], (int, type(None))) else (array_to_list(a) if a == [] else a) for a in args]' if is_linkedlist else ''}
        {'args = [array_to_graph(a) if isinstance(a, list) else a for a in args]' if is_graph else ''}
        {'args = [array_to_tree(a) if isinstance(a, list) else a for a in args]' if is_tree else ''}
        {'args = [array_to_tree(args[0]), array_to_tree(args[1] if len(args) > 1 else test.get("q", []))]' if is_two_tree else ''}
        {'if len(args) == 3: args = [args[0], find_tree_node(args[0], args[1]), find_tree_node(args[0], args[2])]' if is_lca else ''}
        {'args = [array_to_random_list(a) if isinstance(a, list) and len(a) > 0 and isinstance(a[0], list) else a for a in args]' if is_random_list else ''}

        result = {func_name}(*args)

        {'result = args[0] if result is None else result' if is_in_place else ''}
        {'result = list_to_array(result) if result is not None and hasattr(result, "val") and hasattr(result, "next") else ([] if result is None else result)' if is_linkedlist_output else ''}
        {'result = graph_to_array(result) if result and hasattr(result, "neighbors") else ([] if result is None else result)' if is_graph else ''}
        {'result = tree_to_array(result) if result and hasattr(result, "val") and hasattr(result, "left") else result' if is_tree_output else ''}
        {'result = result.val if result and hasattr(result, "val") else result' if is_lca else ''}
        {'result = random_list_to_array(result) if result and hasattr(result, "random") else ([] if result is None else result)' if is_random_list else ''}

        if isinstance(expected, float) and isinstance(result, float):
            passed = abs(expected - result) < 1e-5
        else:
            passed = compare(expected, result, {is_order_independent}, {is_multi_answer})
        results.append({{'input': test.get('input'), 'expected': expected, 'got': result, 'pass': passed}})
    except Exception as e:
        results.append({{'input': test.get('input'), 'expected': expected, 'got': None, 'error': str(e), 'pass': False}})

print(json.dumps(results))
'''

    stdout, stderr, rc = run_subprocess(['python3'], test_code, '.py')
    if rc == 0 and stdout.strip():
        try:
            return json.loads(stdout.strip())
        except:
            return [{'error': f'Parse error: {stdout[:200]}', 'pass': False}]
    return [{'error': stderr[:500] or 'Unknown error', 'pass': False}]


def run_python_class_test(solution_file: Path, class_name: str, test_cases: list) -> list:
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
        obj = {class_name}(*args_list[0])
        result_list = [None]

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

    stdout, stderr, rc = run_subprocess(['python3'], test_code, '.py')
    if rc == 0 and stdout.strip():
        try:
            return json.loads(stdout.strip())
        except:
            return [{'error': f'Parse error', 'pass': False}]
    return [{'error': stderr[:500] or 'Unknown error', 'pass': False}]


def run_python_roundtrip_test(solution_file: Path, func_name: str, test_cases: list) -> list:
    """Run encode/decode or serialize/deserialize tests."""
    with open(solution_file) as f:
        solution_code = f.read()

    if func_name == 'encodeDecode':
        runner = 'result = decode(encode(test_input[0]))'
    else:
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

    stdout, stderr, rc = run_subprocess(['python3'], test_code, '.py')
    if rc == 0 and stdout.strip():
        try:
            return json.loads(stdout.strip())
        except:
            return [{'error': f'Parse error', 'pass': False}]
    return [{'error': stderr[:500] or 'Unknown error', 'pass': False}]


def test_python(problem_id: str, solution_file: Path) -> TestResult:
    """Test a Python solution."""
    start = time.time()

    problem_info = get_problem_info(problem_id)
    test_cases = get_test_cases(problem_id)

    if not problem_info or not test_cases:
        return TestResult(problem_id, 'python', error="Missing files")

    func_name = problem_info.get('function_name', '')

    if func_name in CLASS_FUNCS:
        results = run_python_class_test(solution_file, CLASS_FUNCS[func_name], test_cases)
    elif func_name in ROUNDTRIP_FUNCS:
        results = run_python_roundtrip_test(solution_file, func_name, test_cases)
    else:
        results = run_python_test(solution_file, func_name, test_cases)

    passed = sum(1 for r in results if r.get('pass'))
    total = len(results)
    failures = [r for r in results if not r.get('pass')][:3]
    duration = (time.time() - start) * 1000

    return TestResult(problem_id, 'python', passed, total, failures, None, duration)


# ============================================================================
# JavaScript Runner
# ============================================================================

JS_HELPERS = '''
const arrayToList = (arr) => {
    if (!arr || arr.length === 0) return null;
    let head = { val: arr[0], next: null };
    let curr = head;
    for (let i = 1; i < arr.length; i++) {
        curr.next = { val: arr[i], next: null };
        curr = curr.next;
    }
    return head;
};

const listToArray = (head) => {
    const result = [];
    while (head) {
        result.push(head.val);
        head = head.next;
    }
    return result;
};

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
        if (node) {
            result.push(node.val);
            queue.push(node.left);
            queue.push(node.right);
        } else {
            result.push(null);
        }
    }
    while (result.length > 0 && result[result.length - 1] === null) result.pop();
    return result;
};

const findNode = (root, val) => {
    if (!root) return null;
    if (root.val === val) return root;
    return findNode(root.left, val) || findNode(root.right, val);
};

const buildGraph = (adjList) => {
    if (!adjList || adjList.length === 0) return null;
    const nodes = adjList.map((_, i) => ({ val: i + 1, neighbors: [] }));
    adjList.forEach((neighbors, i) => {
        nodes[i].neighbors = neighbors.map(n => nodes[n - 1]);
    });
    return nodes[0];
};

const graphToAdj = (node) => {
    if (!node) return [];
    const visited = new Map();
    const nodes = [];
    const queue = [node];
    visited.set(node.val, true);
    while (queue.length > 0) {
        const curr = queue.shift();
        nodes.push(curr);
        for (const neighbor of curr.neighbors) {
            if (!visited.has(neighbor.val)) {
                visited.set(neighbor.val, true);
                queue.push(neighbor);
            }
        }
    }
    nodes.sort((a, b) => a.val - b.val);
    return nodes.map(n => n.neighbors.map(nb => nb.val).sort((a, b) => a - b));
};

const buildCyclicList = (arr, pos) => {
    if (!arr || arr.length === 0) return null;
    const nodes = arr.map(v => ({ val: v, next: null }));
    for (let i = 0; i < nodes.length - 1; i++) nodes[i].next = nodes[i + 1];
    if (pos >= 0 && pos < nodes.length) nodes[nodes.length - 1].next = nodes[pos];
    return nodes[0];
};

const buildRandomList = (arr) => {
    if (!arr || arr.length === 0) return null;
    const nodes = arr.map(([val]) => ({ val, next: null, random: null }));
    arr.forEach(([val, randomIdx], i) => {
        if (i < arr.length - 1) nodes[i].next = nodes[i + 1];
        if (randomIdx !== null) nodes[i].random = nodes[randomIdx];
    });
    return nodes[0];
};

const randomListToArr = (head) => {
    if (!head) return [];
    const result = [];
    const nodeToIdx = new Map();
    let curr = head;
    let idx = 0;
    while (curr) {
        nodeToIdx.set(curr, idx++);
        curr = curr.next;
    }
    curr = head;
    while (curr) {
        const randomIdx = curr.random ? nodeToIdx.get(curr.random) : null;
        result.push([curr.val, randomIdx]);
        curr = curr.next;
    }
    return result;
};

const normalize = (val) => {
    if (Array.isArray(val)) {
        return [...val].map(normalize).sort((a, b) => JSON.stringify(a).localeCompare(JSON.stringify(b)));
    }
    return val;
};
'''

# JS function categories
JS_CLASS_FUNCS = {
    'LRUCache': 'LRUCache', 'MinStack': 'MinStack', 'trie': 'Trie',
    'wordDictionary': 'WordDictionary', 'medianFinder': 'MedianFinder',
    'Twitter': 'Twitter', 'KthLargest': 'KthLargest', 'TimeMap': 'TimeMap',
    'DetectSquares': 'DetectSquares'
}
JS_LINKEDLIST_FUNCS = {'addTwoNumbers', 'removeNthFromEnd', 'mergeTwoLists', 'reverseKGroup', 'reverseList', 'reorderList'}
JS_LINKEDLIST_OUTPUT = JS_LINKEDLIST_FUNCS
JS_TREE_FUNCS = {'invertTree', 'maxDepth', 'diameterOfBinaryTree', 'isBalanced', 'isSubtree', 'levelOrder', 'rightSideView', 'isValidBST', 'kthSmallest', 'maxPathSum', 'goodNodes'}
JS_TREE_OUTPUT_FUNCS = {'invertTree', 'buildTree'}
JS_ORDER_INDEPENDENT = {'subsets', 'subsetsWithDup', 'permute', 'permuteUnique', 'combinationSum', 'combinationSum2', 'threeSum', 'letterCombinations', 'generateParenthesis', 'partition', 'solveNQueens', 'groupAnagrams', 'findWords', 'pacificAtlantic'}
JS_IN_PLACE_FUNCS = {'rotate', 'setZeroes', 'solve', 'wallsAndGates'}
JS_LINKEDLIST_IN_PLACE = {'reorderList'}  # In-place linked list modifications
JS_TWO_TREE_FUNCS = {'isSameTree'}
JS_MULTI_ANSWER_FUNCS = {'longestPalindrome'}  # Multiple valid answers


def run_js_test(solution_file: Path, func_name: str, test_cases: list) -> list:
    """Run tests for a JavaScript solution."""
    with open(solution_file) as f:
        solution_code = f.read()

    is_linkedlist = func_name in JS_LINKEDLIST_FUNCS
    is_tree = func_name in JS_TREE_FUNCS
    is_tree_output = func_name in JS_TREE_OUTPUT_FUNCS
    is_linkedlist_output = func_name in JS_LINKEDLIST_OUTPUT
    is_order_independent = func_name in JS_ORDER_INDEPENDENT
    is_in_place = func_name in JS_IN_PLACE_FUNCS
    is_two_tree = func_name in JS_TWO_TREE_FUNCS

    test_code = f'''
{solution_code}

{JS_HELPERS}

const testCases = {json.dumps(test_cases)};
const results = [];

for (const tc of testCases) {{
    try {{
        let args;
        if (typeof tc.input === 'object' && !Array.isArray(tc.input)) {{
            args = Object.values(tc.input);
        }} else {{
            args = Array.isArray(tc.input) ? [...tc.input] : [tc.input];
        }}

        {'args = args.map(a => Array.isArray(a) ? arrayToList(a) : a);' if is_linkedlist else ''}
        {'args = args.map(a => Array.isArray(a) ? arrayToTree(a) : a);' if is_tree else ''}
        {'args = [arrayToTree(args[0]), arrayToTree(args[1] || [])];' if is_two_tree else ''}

        let result = {func_name}(...args);

        {'result = result === undefined || result === null ? args[0] : result;' if is_in_place else ''}
        {'result = result && result.next !== undefined ? listToArray(result) : (result === null ? [] : result);' if is_linkedlist_output else ''}
        {'result = result && result.left !== undefined ? treeToArray(result) : result;' if is_tree_output else ''}

        let pass;
        // Float comparison with tolerance
        const floatCompare = (a, b) => typeof a === 'number' && typeof b === 'number' && Math.abs(a - b) < 1e-5;
        if ({'true' if is_order_independent else 'false'}) {{
            pass = JSON.stringify(normalize(result)) === JSON.stringify(normalize(tc.expected));
        }} else if (floatCompare(result, tc.expected)) {{
            pass = true;
        }} else {{
            pass = JSON.stringify(result) === JSON.stringify(tc.expected);
        }}
        results.push({{ input: tc.input, expected: tc.expected, got: result, pass }});
    }} catch (e) {{
        results.push({{ input: tc.input, expected: tc.expected, error: e.message, pass: false }});
    }}
}}

console.log(JSON.stringify(results));
'''

    stdout, stderr, rc = run_subprocess(['bun', 'run'], test_code, '.js')
    if rc == 0 and stdout.strip():
        try:
            return json.loads(stdout.strip())
        except:
            return [{'error': f'Parse error: {stdout[:200]}', 'pass': False}]
    return [{'error': stderr[:500] or 'Unknown error', 'pass': False}]


def run_js_class_test(solution_file: Path, class_name: str, test_cases: list) -> list:
    """Run tests for class-based JavaScript solutions."""
    with open(solution_file) as f:
        solution_code = f.read()

    test_code = f'''
{solution_code}

const testCases = {json.dumps(test_cases)};
const results = [];

for (const tc of testCases) {{
    try {{
        const [methods, args] = tc.input;
        const outputs = [];
        let instance = null;

        for (let i = 0; i < methods.length; i++) {{
            const method = methods[i];
            const arg = args[i];

            if (method === '{class_name}') {{
                instance = new {class_name}(...arg);
                outputs.push(null);
            }} else {{
                const result = instance[method](...arg);
                outputs.push(result === undefined ? null : result);
            }}
        }}

        const pass = JSON.stringify(outputs) === JSON.stringify(tc.expected);
        results.push({{ input: tc.input, expected: tc.expected, got: outputs, pass }});
    }} catch (e) {{
        results.push({{ input: tc.input, expected: tc.expected, error: e.message, pass: false }});
    }}
}}

console.log(JSON.stringify(results));
'''

    stdout, stderr, rc = run_subprocess(['bun', 'run'], test_code, '.js')
    if rc == 0 and stdout.strip():
        try:
            return json.loads(stdout.strip())
        except:
            return [{'error': f'Parse error', 'pass': False}]
    return [{'error': stderr[:500] or 'Unknown error', 'pass': False}]


def run_js_special_test(solution_file: Path, func_name: str, test_cases: list) -> list:
    """Run special JS tests (hasCycle, mergeKLists, cloneGraph, etc.)."""
    with open(solution_file) as f:
        solution_code = f.read()

    if func_name == 'hasCycle':
        runner = '''
        const [arr, pos] = Array.isArray(tc.input) ? tc.input : Object.values(tc.input);
        const list = buildCyclicList(arr, pos);
        result = hasCycle(list);
        pass = result === tc.expected;
        '''
    elif func_name == 'mergeKLists':
        runner = '''
        const listsArr = Array.isArray(tc.input[0]) ? tc.input[0] : tc.input;
        const lists = listsArr.map(arr => arrayToList(arr));
        result = listToArray(mergeKLists(lists));
        pass = JSON.stringify(result) === JSON.stringify(tc.expected);
        '''
    elif func_name == 'cloneGraph':
        runner = '''
        const adjList = tc.input.adjList !== undefined ? tc.input.adjList : tc.input[0];
        const graph = buildGraph(adjList);
        const cloned = cloneGraph(graph);
        result = graphToAdj(cloned);
        pass = JSON.stringify(result) === JSON.stringify(tc.expected);
        '''
    elif func_name == 'copyRandomList':
        runner = '''
        const inputArr = Array.isArray(tc.input[0]) ? tc.input[0] : tc.input;
        const list = buildRandomList(inputArr);
        const copied = copyRandomList(list);
        result = randomListToArr(copied);
        pass = JSON.stringify(result) === JSON.stringify(tc.expected);
        '''
    elif func_name == 'lowestCommonAncestor':
        runner = '''
        const { root: rootArr, p: pVal, q: qVal } = tc.input;
        const root = arrayToTree(rootArr);
        const p = findNode(root, pVal);
        const q = findNode(root, qVal);
        const lca = lowestCommonAncestor(root, p, q);
        result = lca ? lca.val : null;
        pass = result === tc.expected;
        '''
    elif func_name == 'codec':
        runner = '''
        const inputArr = Array.isArray(tc.input) ? tc.input[0] : tc.input;
        const tree = arrayToTree(inputArr);
        const serialized = serialize(tree);
        const deserialized = deserialize(serialized);
        result = treeToArray(deserialized);
        pass = JSON.stringify(result) === JSON.stringify(tc.expected);
        '''
    elif func_name == 'encodeDecode':
        runner = '''
        let inputArr = tc.input;
        while (Array.isArray(inputArr) && inputArr.length === 1 && Array.isArray(inputArr[0])) {
            inputArr = inputArr[0];
        }
        const encoded = encode(inputArr);
        result = decode(encoded);
        pass = JSON.stringify(result) === JSON.stringify(tc.expected);
        '''
    elif func_name == 'reorderList':
        runner = '''
        const inputArr = Array.isArray(tc.input) ? tc.input[0] : (tc.input.head || tc.input);
        const head = arrayToList(inputArr);
        reorderList(head);
        result = listToArray(head);
        pass = JSON.stringify(result) === JSON.stringify(tc.expected);
        '''
    elif func_name == 'longestPalindrome':
        runner = '''
        const s = typeof tc.input === 'object' ? Object.values(tc.input)[0] : tc.input;
        result = longestPalindrome(s);
        // Expected is array of valid answers, check if result is in it
        pass = Array.isArray(tc.expected) ? tc.expected.includes(result) : result === tc.expected;
        '''
    else:
        return [{'error': f'Unknown special function: {func_name}', 'pass': False}]

    test_code = f'''
{solution_code}

{JS_HELPERS}

const testCases = {json.dumps(test_cases)};
const results = [];

for (const tc of testCases) {{
    try {{
        let result, pass;
        {runner}
        results.push({{ input: tc.input, expected: tc.expected, got: result, pass }});
    }} catch (e) {{
        results.push({{ input: tc.input, expected: tc.expected, error: e.message, pass: false }});
    }}
}}

console.log(JSON.stringify(results));
'''

    stdout, stderr, rc = run_subprocess(['bun', 'run'], test_code, '.js')
    if rc == 0 and stdout.strip():
        try:
            return json.loads(stdout.strip())
        except:
            return [{'error': f'Parse error', 'pass': False}]
    return [{'error': stderr[:500] or 'Unknown error', 'pass': False}]


JS_SPECIAL_FUNCS = {'hasCycle', 'mergeKLists', 'cloneGraph', 'copyRandomList', 'lowestCommonAncestor', 'codec', 'encodeDecode', 'reorderList', 'longestPalindrome'}


def test_javascript(problem_id: str, solution_file: Path) -> TestResult:
    """Test a JavaScript solution."""
    start = time.time()

    problem_info = get_problem_info(problem_id)
    test_cases = get_test_cases(problem_id)

    if not problem_info or not test_cases:
        return TestResult(problem_id, 'javascript', error="Missing files")

    func_name = problem_info.get('function_name', '')

    if func_name in JS_CLASS_FUNCS:
        results = run_js_class_test(solution_file, JS_CLASS_FUNCS[func_name], test_cases)
    elif func_name in JS_SPECIAL_FUNCS:
        results = run_js_special_test(solution_file, func_name, test_cases)
    else:
        results = run_js_test(solution_file, func_name, test_cases)

    passed = sum(1 for r in results if r.get('pass'))
    total = len(results)
    failures = [r for r in results if not r.get('pass')][:3]
    duration = (time.time() - start) * 1000

    return TestResult(problem_id, 'javascript', passed, total, failures, None, duration)


# ============================================================================
# C++ Runner
# ============================================================================

CPP_HELPERS = '''
#include <iostream>
#include <vector>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <map>
#include <set>
#include <queue>
#include <stack>
#include <list>
#include <algorithm>
#include <climits>
#include <cmath>
#include <sstream>
#include <iomanip>
#include <functional>
#include <numeric>

using namespace std;

// Linked list node
struct ListNode {
    int val;
    ListNode* next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode* n) : val(x), next(n) {}
};

// Tree node
struct TreeNode {
    int val;
    TreeNode* left;
    TreeNode* right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode* l, TreeNode* r) : val(x), left(l), right(r) {}
};

// Random pointer list node (for problem 138) - named RandomNode to avoid conflicts with graph Node
struct RandomNode {
    int val;
    RandomNode* next;
    RandomNode* random;
    RandomNode(int _val) : val(_val), next(nullptr), random(nullptr) {}
};

// Convert vector to linked list
ListNode* vectorToList(const vector<int>& arr) {
    if (arr.empty()) return nullptr;
    ListNode* head = new ListNode(arr[0]);
    ListNode* curr = head;
    for (size_t i = 1; i < arr.size(); i++) {
        curr->next = new ListNode(arr[i]);
        curr = curr->next;
    }
    return head;
}

// Convert linked list to vector
vector<int> listToVector(ListNode* head) {
    vector<int> result;
    while (head) {
        result.push_back(head->val);
        head = head->next;
    }
    return result;
}

// Find a node by value in a tree (for LCA problems)
TreeNode* findNode(TreeNode* root, int val) {
    if (!root) return nullptr;
    if (root->val == val) return root;
    TreeNode* left = findNode(root->left, val);
    if (left) return left;
    return findNode(root->right, val);
}

// Create random list from array of [val, random_index] pairs
RandomNode* createRandomList(const vector<pair<int, int>>& arr) {
    if (arr.empty()) return nullptr;
    vector<RandomNode*> nodes;
    for (const auto& p : arr) {
        nodes.push_back(new RandomNode(p.first));
    }
    for (size_t i = 0; i < nodes.size(); i++) {
        if (i < nodes.size() - 1) nodes[i]->next = nodes[i + 1];
        if (arr[i].second >= 0 && arr[i].second < (int)nodes.size()) {
            nodes[i]->random = nodes[arr[i].second];
        }
    }
    return nodes[0];
}

// Convert random list to JSON array
string randomListToJson(RandomNode* head) {
    if (!head) return "[]";
    vector<RandomNode*> nodes;
    map<RandomNode*, int> nodeToIdx;
    RandomNode* curr = head;
    int idx = 0;
    while (curr) {
        nodeToIdx[curr] = idx++;
        nodes.push_back(curr);
        curr = curr->next;
    }
    string result = "[";
    for (size_t i = 0; i < nodes.size(); i++) {
        if (i > 0) result += ",";
        result += "[" + to_string(nodes[i]->val) + ",";
        if (nodes[i]->random) {
            result += to_string(nodeToIdx[nodes[i]->random]);
        } else {
            result += "null";
        }
        result += "]";
    }
    return result + "]";
}

// Create linked list with cycle at position pos (-1 for no cycle)
ListNode* createCycleList(const vector<int>& arr, int pos) {
    if (arr.empty()) return nullptr;
    vector<ListNode*> nodes;
    for (int val : arr) {
        nodes.push_back(new ListNode(val));
    }
    for (size_t i = 0; i < nodes.size() - 1; i++) {
        nodes[i]->next = nodes[i + 1];
    }
    if (pos >= 0 && pos < (int)nodes.size()) {
        nodes.back()->next = nodes[pos];
    }
    return nodes[0];
}

// Special marker for null tree nodes (using LLONG_MIN since tree values are int)
const long long TREE_NULL = LLONG_MIN;

// Convert vector to tree (level order) - uses LLONG_MIN as null marker
TreeNode* vectorToTree(const vector<long long>& arr) {
    if (arr.empty() || arr[0] == TREE_NULL) return nullptr;
    TreeNode* root = new TreeNode((int)arr[0]);
    queue<TreeNode*> q;
    q.push(root);
    size_t i = 1;
    while (!q.empty() && i < arr.size()) {
        TreeNode* node = q.front();
        q.pop();
        if (i < arr.size() && arr[i] != TREE_NULL) {
            node->left = new TreeNode((int)arr[i]);
            q.push(node->left);
        }
        i++;
        if (i < arr.size() && arr[i] != TREE_NULL) {
            node->right = new TreeNode((int)arr[i]);
            q.push(node->right);
        }
        i++;
    }
    return root;
}

// Convert tree to vector (level order)
vector<int> treeToVector(TreeNode* root) {
    if (!root) return {};
    vector<int> result;
    queue<TreeNode*> q;
    q.push(root);
    while (!q.empty()) {
        TreeNode* node = q.front();
        q.pop();
        if (node) {
            result.push_back(node->val);
            q.push(node->left);
            q.push(node->right);
        } else {
            result.push_back(INT_MIN); // null marker
        }
    }
    // Remove trailing nulls
    while (!result.empty() && result.back() == INT_MIN) {
        result.pop_back();
    }
    return result;
}

// Convert tree vector to JSON with null for INT_MIN
string treeVectorToJson(const vector<int>& arr) {
    string result = "[";
    for (size_t i = 0; i < arr.size(); i++) {
        if (i > 0) result += ",";
        if (arr[i] == INT_MIN) {
            result += "null";
        } else {
            result += to_string(arr[i]);
        }
    }
    return result + "]";
}

// JSON parsing helpers
string trim(const string& s) {
    size_t start = s.find_first_not_of(" \\t\\n\\r");
    if (start == string::npos) return "";
    size_t end = s.find_last_not_of(" \\t\\n\\r");
    return s.substr(start, end - start + 1);
}

// Simple JSON array parser for integers
vector<int> parseIntArray(const string& s) {
    vector<int> result;
    string trimmed = trim(s);
    if (trimmed.empty() || trimmed == "[]") return result;
    if (trimmed[0] == '[') trimmed = trimmed.substr(1);
    if (!trimmed.empty() && trimmed.back() == ']') trimmed.pop_back();

    stringstream ss(trimmed);
    string token;
    while (getline(ss, token, ',')) {
        string t = trim(token);
        if (!t.empty() && t != "null") {
            try {
                result.push_back(stoi(t));
            } catch (...) {}
        }
    }
    return result;
}

// Parse 2D int array
vector<vector<int>> parse2DIntArray(const string& s) {
    vector<vector<int>> result;
    string trimmed = trim(s);
    if (trimmed.empty() || trimmed == "[]") return result;

    int depth = 0;
    string current;
    for (size_t i = 0; i < trimmed.size(); i++) {
        char c = trimmed[i];
        if (c == '[') {
            depth++;
            if (depth == 2) current = "";
        } else if (c == ']') {
            if (depth == 2 && !current.empty()) {
                result.push_back(parseIntArray("[" + current + "]"));
            }
            depth--;
        } else if (depth == 2) {
            current += c;
        }
    }
    return result;
}

// Parse string array
vector<string> parseStringArray(const string& s) {
    vector<string> result;
    string trimmed = trim(s);
    if (trimmed.empty() || trimmed == "[]") return result;

    bool inString = false;
    string current;
    bool escaped = false;

    for (size_t i = 1; i < trimmed.size() - 1; i++) {
        char c = trimmed[i];
        if (escaped) {
            current += c;
            escaped = false;
        } else if (c == '\\\\') {
            escaped = true;
        } else if (c == '"') {
            if (inString) {
                result.push_back(current);
                current = "";
            }
            inString = !inString;
        } else if (inString) {
            current += c;
        }
    }
    return result;
}

// Parse 2D string array
vector<vector<string>> parse2DStringArray(const string& s) {
    vector<vector<string>> result;
    string trimmed = trim(s);
    if (trimmed.empty() || trimmed == "[]") return result;

    int depth = 0;
    string current;
    for (size_t i = 0; i < trimmed.size(); i++) {
        char c = trimmed[i];
        if (c == '[') {
            depth++;
            if (depth == 2) current = "[";
        } else if (c == ']') {
            if (depth == 2) {
                current += "]";
                result.push_back(parseStringArray(current));
            }
            depth--;
        } else if (depth >= 2) {
            current += c;
        }
    }
    return result;
}

// Convert to JSON string
string toJson(int val) { return to_string(val); }
string toJson(long long val) { return to_string(val); }
string toJson(double val) {
    // Check if it's an integer value
    if (val == (long long)val) {
        return to_string((long long)val);
    }
    stringstream ss;
    ss << fixed << setprecision(5) << val;
    string s = ss.str();
    // Remove trailing zeros but keep at least one decimal place
    size_t dot = s.find('.');
    if (dot != string::npos) {
        size_t last = s.find_last_not_of('0');
        if (last > dot) s = s.substr(0, last + 1);
        else s = s.substr(0, dot + 2); // Keep .0
    }
    return s;
}
string toJson(bool val) { return val ? "true" : "false"; }
string toJson(const string& val) {
    string result = "\\"";
    for (char c : val) {
        if (c == '"') result += "\\\\\\"";
        else if (c == '\\\\') result += "\\\\\\\\";
        else result += c;
    }
    return result + "\\"";
}
string toJson(uint32_t val) { return to_string(val); }

string toJson(const vector<int>& arr) {
    string result = "[";
    for (size_t i = 0; i < arr.size(); i++) {
        if (i > 0) result += ",";
        result += to_string(arr[i]);
    }
    return result + "]";
}

string toJson(const vector<bool>& arr) {
    string result = "[";
    for (size_t i = 0; i < arr.size(); i++) {
        if (i > 0) result += ",";
        result += arr[i] ? "true" : "false";
    }
    return result + "]";
}

string toJson(const vector<string>& arr) {
    string result = "[";
    for (size_t i = 0; i < arr.size(); i++) {
        if (i > 0) result += ",";
        result += toJson(arr[i]);
    }
    return result + "]";
}

string toJson(const vector<vector<int>>& arr) {
    string result = "[";
    for (size_t i = 0; i < arr.size(); i++) {
        if (i > 0) result += ",";
        result += toJson(arr[i]);
    }
    return result + "]";
}

string toJson(const vector<vector<string>>& arr) {
    string result = "[";
    for (size_t i = 0; i < arr.size(); i++) {
        if (i > 0) result += ",";
        result += toJson(arr[i]);
    }
    return result + "]";
}

// Normalization for order-independent comparison
string normalizeJson(const string& s) {
    // Parse as array, sort inner elements, sort outer by string representation
    string trimmed = trim(s);
    if (trimmed.empty() || trimmed[0] != '[') return trimmed;

    // Check if it contains strings (has quotes)
    bool hasStrings = trimmed.find('"') != string::npos;

    // For simple arrays (no nested brackets)
    if (trimmed.find('[', 1) == string::npos) {
        if (hasStrings) {
            vector<string> arr = parseStringArray(trimmed);
            sort(arr.begin(), arr.end());
            return toJson(arr);
        } else {
            vector<int> arr = parseIntArray(trimmed);
            sort(arr.begin(), arr.end());
            return toJson(arr);
        }
    }

    // For 2D arrays - parse each inner array, sort it, then sort outer by string representation
    if (hasStrings) {
        vector<vector<string>> arr = parse2DStringArray(trimmed);
        vector<string> strs;
        for (auto& inner : arr) {
            sort(inner.begin(), inner.end());
            strs.push_back(toJson(inner));
        }
        sort(strs.begin(), strs.end());
        string result = "[";
        for (size_t i = 0; i < strs.size(); i++) {
            if (i > 0) result += ",";
            result += strs[i];
        }
        return result + "]";
    } else {
        vector<vector<int>> arr = parse2DIntArray(trimmed);
        vector<string> strs;
        for (auto& inner : arr) {
            sort(inner.begin(), inner.end());
            strs.push_back(toJson(inner));
        }
        sort(strs.begin(), strs.end());
        string result = "[";
        for (size_t i = 0; i < strs.size(); i++) {
            if (i > 0) result += ",";
            result += strs[i];
        }
        return result + "]";
    }
}
'''

# C++ function categories (similar to Python/JS)
CPP_ORDER_INDEPENDENT = {'subsets', 'subsetsWithDup', 'permute', 'permuteUnique', 'combinationSum', 'combinationSum2', 'threeSum', 'letterCombinations', 'generateParenthesis', 'partition', 'solveNQueens', 'groupAnagrams', 'findWords', 'pacificAtlantic', 'topKFrequent'}
CPP_MULTI_ANSWER = {'longestPalindrome'}  # Functions where multiple answers are valid
CPP_CLASS_FUNCS = {'LRUCache': 'LRUCache', 'MinStack': 'MinStack', 'KthLargest': 'KthLargest', 'MedianFinder': 'MedianFinder', 'TimeMap': 'TimeMap', 'Trie': 'Trie', 'WordDictionary': 'WordDictionary', 'DetectSquares': 'DetectSquares', 'Codec': 'Codec', 'Twitter': 'Twitter'}
CPP_CYCLE_FUNCS = {'hasCycle'}  # Need special cycle list creation
CPP_INPLACE_LIST_FUNCS = {'reorderList'}  # Void return, modifies list in place
CPP_RANDOM_LIST_FUNCS = {'copyRandomList'}  # Special Node* with random pointer
CPP_LCA_FUNCS = {'lowestCommonAncestor'}  # Need to find nodes by value in tree


def generate_cpp_harness(solution_code: str, func_name: str, test_cases: list, is_order_independent: bool = False, is_multi_answer: bool = False) -> str:
    """Generate a C++ test harness for the solution."""

    # Detect function signature from solution code
    # Look for the method in class Solution
    import re

    # Match return type including nested templates like vector<vector<string>> and pointers like ListNode*
    # Pattern handles nested angle brackets and pointer types
    pattern = r'((?:\w+(?:<(?:[^<>]|<[^<>]*>)*>)?)+\*?)\s+' + re.escape(func_name) + r'\s*\(([^)]*)\)'
    match = re.search(pattern, solution_code)
    if not match:
        return None

    return_type = match.group(1).strip()
    params_str = match.group(2).strip()

    # Parse parameters
    params = []
    if params_str:
        # Split by comma but respect template brackets
        depth = 0
        current = ""
        for c in params_str:
            if c in '<':
                depth += 1
            elif c in '>':
                depth -= 1
            elif c == ',' and depth == 0:
                params.append(current.strip())
                current = ""
                continue
            current += c
        if current.strip():
            params.append(current.strip())

    # Generate test code
    test_code = CPP_HELPERS + '\n' + solution_code + '\n\n'
    test_code += 'int main() {\n'
    test_code += '    Solution sol;\n'
    test_code += '    cout << "[";\n'
    test_code += '    bool first = true;\n\n'

    for i, tc in enumerate(test_cases):
        inp = tc.get('input', {})
        expected = tc.get('expected')

        # Convert input to argument values
        if isinstance(inp, dict):
            args = list(inp.values())
        elif isinstance(inp, list):
            args = inp
        else:
            args = [inp]

        test_code += f'    // Test case {i}\n'
        test_code += '    {\n'
        test_code += '        if (!first) cout << ",";\n'
        test_code += '        first = false;\n'
        test_code += '        try {\n'

        # Declare variables for each argument
        arg_names = []
        for j, (arg, param) in enumerate(zip(args, params)):
            param_parts = param.rsplit(None, 1)
            if len(param_parts) >= 2:
                param_type = param_parts[0].replace('&', '').strip()
                var_name = f'arg{j}'
                arg_names.append(var_name)

                # Initialize based on type
                if 'vector<ListNode*>' in param_type:
                    # Convert array of arrays to vector of linked lists
                    test_code += f'            vector<ListNode*> {var_name};\n'
                    for i, lst in enumerate(arg if arg else []):
                        arr_str = ','.join(map(str, lst)) if lst else ''
                        test_code += f'            {var_name}.push_back(vectorToList({{{arr_str}}}));\n'
                elif 'ListNode*' in param_type or 'ListNode *' in param_type:
                    # Convert input array to linked list
                    arr_str = ','.join(map(str, arg)) if arg else ''
                    test_code += f'            ListNode* {var_name} = vectorToList({{{arr_str}}});\n'
                elif 'TreeNode*' in param_type or 'TreeNode *' in param_type:
                    # Convert input array to tree (use LLONG_MIN for null nodes)
                    if arg:
                        arr_vals = [str(x) + 'LL' if x is not None else 'TREE_NULL' for x in arg]
                        arr_str = ','.join(arr_vals)
                    else:
                        arr_str = ''
                    test_code += f'            TreeNode* {var_name} = vectorToTree({{{arr_str}}});\n'
                elif 'vector<vector<int>>' in param_type:
                    inner = ','.join('{' + ','.join(map(str, row)) + '}' for row in arg) if arg else ''
                    test_code += f'            vector<vector<int>> {var_name} = {{{inner}}};\n'
                elif 'vector<int>' in param_type:
                    test_code += f'            vector<int> {var_name} = {{{",".join(map(str, arg)) if arg else ""}}};\n'
                elif 'vector<vector<char>>' in param_type:
                    # Handle 2D char array (like Sudoku boards, islands)
                    # Input values may be single chars or single-char strings
                    def to_char(c):
                        if isinstance(c, str) and len(c) == 1:
                            return f"'{c}'"
                        return f"'{c}'"
                    inner = ','.join('{' + ','.join(to_char(c) for c in row) + '}' for row in arg) if arg else ''
                    test_code += f'            vector<vector<char>> {var_name} = {{{inner}}};\n'
                elif 'vector<vector<string>>' in param_type:
                    inner = ','.join('{' + ','.join(f'"{s}"' for s in row) + '}' for row in arg) if arg else ''
                    test_code += f'            vector<vector<string>> {var_name} = {{{inner}}};\n'
                elif 'vector<char>' in param_type:
                    inner = ','.join(f"'{c}'" for c in arg) if arg else ''
                    test_code += f'            vector<char> {var_name} = {{{inner}}};\n'
                elif 'vector<string>' in param_type:
                    inner = ','.join(f'"{s}"' for s in arg) if arg else ''
                    test_code += f'            vector<string> {var_name} = {{{inner}}};\n'
                elif 'string' in param_type:
                    escaped = str(arg).replace('\\', '\\\\').replace('"', '\\"')
                    test_code += f'            string {var_name} = "{escaped}";\n'
                elif 'int' in param_type or 'long' in param_type:
                    test_code += f'            {param_type} {var_name} = {arg};\n'
                elif 'uint32_t' in param_type:
                    test_code += f'            uint32_t {var_name} = {arg}u;\n'
                elif 'char' in param_type:
                    test_code += f'            char {var_name} = \'{arg}\';\n'
                elif 'bool' in param_type:
                    test_code += f'            bool {var_name} = {"true" if arg else "false"};\n'
                elif 'double' in param_type or 'float' in param_type:
                    test_code += f'            {param_type} {var_name} = {arg};\n'
                else:
                    # Default to int
                    test_code += f'            int {var_name} = {arg};\n'

        # Call function
        call_args = ', '.join(arg_names)
        test_code += f'            auto result = sol.{func_name}({call_args});\n'

        # Convert result based on return type
        if return_type == 'ListNode*':
            test_code += '            auto resultVec = listToVector(result);\n'
            test_code += '            string got = toJson(resultVec);\n'
        elif return_type == 'TreeNode*':
            test_code += '            auto resultVec = treeToVector(result);\n'
            test_code += '            string got = treeVectorToJson(resultVec);\n'
        else:
            test_code += '            string got = toJson(result);\n'

        # Convert expected to string for comparison
        # Use custom delimiter to handle strings containing )"
        expected_json = json.dumps(expected, separators=(',', ':'))
        test_code += f'            string expected = R"JSON({expected_json})JSON";\n'

        if is_multi_answer and isinstance(expected, list):
            # For multi-answer, check if got is in expected array
            test_code += '            bool pass = false;\n'
            test_code += '            // Check if result is one of the valid answers\n'
            for valid_answer in expected:
                valid_json = json.dumps(valid_answer, separators=(',', ':'))
                test_code += f'            if (got == R"JSON({valid_json})JSON") pass = true;\n'
        elif is_order_independent:
            test_code += '            bool pass = normalizeJson(got) == normalizeJson(expected);\n'
        elif isinstance(expected, float):
            # Float comparison with tolerance
            test_code += f'            bool pass = abs(result - {expected}) < 1e-5;\n'
        else:
            test_code += '            bool pass = got == expected;\n'

        input_json = json.dumps(tc.get('input'), separators=(',', ':')).replace('\\', '\\\\').replace('"', '\\"')
        test_code += f'            cout << "{{\\"pass\\":" << (pass ? "true" : "false") << ",\\"got\\":" << got << ",\\"expected\\":" << expected << "}}";\n'
        test_code += '        } catch (exception& e) {\n'
        test_code += '            cout << "{\\"pass\\":false,\\"error\\":\\"" << e.what() << "\\"}";\n'
        test_code += '        } catch (...) {\n'
        test_code += '            cout << "{\\"pass\\":false,\\"error\\":\\"unknown error\\"}";\n'
        test_code += '        }\n'
        test_code += '    }\n\n'

    test_code += '    cout << "]" << endl;\n'
    test_code += '    return 0;\n'
    test_code += '}\n'

    return test_code


def generate_cpp_class_harness(solution_code: str, class_name: str, test_cases: list) -> str:
    """Generate a C++ test harness for class-based solutions (LRUCache, MinStack, etc.)."""
    import json

    test_code = CPP_HELPERS + '\n' + solution_code + '\n\n'
    test_code += 'int main() {\n'
    test_code += '    cout << "[";\n'
    test_code += '    bool first = true;\n\n'

    for i, tc in enumerate(test_cases):
        inp = tc.get('input', [])
        expected = tc.get('expected', [])

        # Input format: [[methods], [args]]
        methods = inp[0]
        args = inp[1]

        test_code += f'    // Test case {i}\n'
        test_code += '    {\n'
        test_code += '        if (!first) cout << ",";\n'
        test_code += '        first = false;\n'
        test_code += '        try {\n'
        test_code += '            vector<string> outputs;\n'
        test_code += f'            {class_name}* instance = nullptr;\n\n'

        for j, (method, arg) in enumerate(zip(methods, args)):
            if method == class_name:
                # Constructor call
                if class_name == 'LRUCache':
                    test_code += f'            instance = new {class_name}({arg[0]});\n'
                elif class_name == 'MinStack':
                    test_code += f'            instance = new {class_name}();\n'
                else:
                    # Generic constructor with int args
                    args_str = ', '.join(map(str, arg))
                    test_code += f'            instance = new {class_name}({args_str});\n'
                test_code += '            outputs.push_back("null");\n'
            else:
                # Method call
                args_str = ', '.join(map(str, arg))
                if method in ('put', 'push', 'pop'):
                    test_code += f'            instance->{method}({args_str});\n'
                    test_code += '            outputs.push_back("null");\n'
                elif method in ('get', 'top', 'getMin'):
                    test_code += '            {\n'
                    test_code += f'                int r = instance->{method}({args_str});\n'
                    # For MinStack getMin/top on empty stack, check sentinel value
                    if class_name == 'MinStack' and method in ('getMin', 'top'):
                        test_code += '                if (r == INT_MAX) outputs.push_back("null");\n'
                        test_code += '                else outputs.push_back(to_string(r));\n'
                    else:
                        test_code += '                outputs.push_back(to_string(r));\n'
                    test_code += '            }\n'
                else:
                    # Generic method - try to handle various return types
                    test_code += '            {\n'
                    test_code += f'                auto r = instance->{method}({args_str});\n'
                    test_code += '                outputs.push_back(to_string(r));\n'
                    test_code += '            }\n'

        test_code += '\n            // Build expected output string\n'
        expected_json = json.dumps(expected, separators=(',', ':'))
        test_code += f'            string expected = R"JSON({expected_json})JSON";\n'

        test_code += '            // Build got output string\n'
        test_code += '            string got = "[";\n'
        test_code += '            for (size_t i = 0; i < outputs.size(); i++) {\n'
        test_code += '                if (i > 0) got += ",";\n'
        test_code += '                got += outputs[i];\n'
        test_code += '            }\n'
        test_code += '            got += "]";\n'

        test_code += '            bool pass = got == expected;\n'
        test_code += '            cout << "{\\"pass\\":" << (pass ? "true" : "false") << ",\\"got\\":" << got << ",\\"expected\\":" << expected << "}";\n'
        test_code += '            delete instance;\n'
        test_code += '        } catch (exception& e) {\n'
        test_code += '            cout << "{\\"pass\\":false,\\"error\\":\\"" << e.what() << "\\"}";\n'
        test_code += '        } catch (...) {\n'
        test_code += '            cout << "{\\"pass\\":false,\\"error\\":\\"unknown error\\"}";\n'
        test_code += '        }\n'
        test_code += '    }\n\n'

    test_code += '    cout << "]" << endl;\n'
    test_code += '    return 0;\n'
    test_code += '}\n'

    return test_code


def generate_cpp_cycle_harness(solution_code: str, func_name: str, test_cases: list) -> str:
    """Generate a C++ test harness for hasCycle (cycle detection in linked list)."""
    import json

    test_code = CPP_HELPERS + '\n' + solution_code + '\n\n'
    test_code += 'int main() {\n'
    test_code += '    Solution sol;\n'
    test_code += '    cout << "[";\n'
    test_code += '    bool first = true;\n\n'

    for i, tc in enumerate(test_cases):
        inp = tc.get('input', [])
        expected = tc.get('expected', False)

        # Input format: [[arr], pos]
        arr = inp[0] if len(inp) > 0 else []
        pos = inp[1] if len(inp) > 1 else -1

        test_code += f'    // Test case {i}\n'
        test_code += '    {\n'
        test_code += '        if (!first) cout << ",";\n'
        test_code += '        first = false;\n'
        test_code += '        try {\n'

        # Create the array values
        arr_str = ','.join(map(str, arr)) if arr else ''
        test_code += f'            ListNode* head = createCycleList({{{arr_str}}}, {pos});\n'
        test_code += f'            bool result = sol.{func_name}(head);\n'

        expected_str = 'true' if expected else 'false'
        test_code += f'            bool expected = {expected_str};\n'
        test_code += '            bool pass = result == expected;\n'
        test_code += '            cout << "{\\"pass\\":" << (pass ? "true" : "false") << ",\\"got\\":" << (result ? "true" : "false") << ",\\"expected\\":" << (expected ? "true" : "false") << "}";\n'
        test_code += '        } catch (exception& e) {\n'
        test_code += '            cout << "{\\"pass\\":false,\\"error\\":\\"" << e.what() << "\\"}";\n'
        test_code += '        } catch (...) {\n'
        test_code += '            cout << "{\\"pass\\":false,\\"error\\":\\"unknown error\\"}";\n'
        test_code += '        }\n'
        test_code += '    }\n\n'

    test_code += '    cout << "]" << endl;\n'
    test_code += '    return 0;\n'
    test_code += '}\n'

    return test_code


def generate_cpp_inplace_list_harness(solution_code: str, func_name: str, test_cases: list) -> str:
    """Generate a C++ test harness for in-place list modification (reorderList)."""
    import json

    test_code = CPP_HELPERS + '\n' + solution_code + '\n\n'
    test_code += 'int main() {\n'
    test_code += '    Solution sol;\n'
    test_code += '    cout << "[";\n'
    test_code += '    bool first = true;\n\n'

    for i, tc in enumerate(test_cases):
        inp = tc.get('input', [])
        expected = tc.get('expected', [])

        # Input format: [[arr]] - single array wrapped
        arr = inp[0] if len(inp) > 0 and isinstance(inp[0], list) else inp

        test_code += f'    // Test case {i}\n'
        test_code += '    {\n'
        test_code += '        if (!first) cout << ",";\n'
        test_code += '        first = false;\n'
        test_code += '        try {\n'

        # Create the list
        arr_str = ','.join(map(str, arr)) if arr else ''
        test_code += f'            ListNode* head = vectorToList({{{arr_str}}});\n'
        test_code += f'            sol.{func_name}(head);\n'
        test_code += '            auto resultVec = listToVector(head);\n'
        test_code += '            string got = toJson(resultVec);\n'

        expected_json = json.dumps(expected, separators=(',', ':'))
        test_code += f'            string expected = R"JSON({expected_json})JSON";\n'
        test_code += '            bool pass = got == expected;\n'
        test_code += '            cout << "{\\"pass\\":" << (pass ? "true" : "false") << ",\\"got\\":" << got << ",\\"expected\\":" << expected << "}";\n'
        test_code += '        } catch (exception& e) {\n'
        test_code += '            cout << "{\\"pass\\":false,\\"error\\":\\"" << e.what() << "\\"}";\n'
        test_code += '        } catch (...) {\n'
        test_code += '            cout << "{\\"pass\\":false,\\"error\\":\\"unknown error\\"}";\n'
        test_code += '        }\n'
        test_code += '    }\n\n'

    test_code += '    cout << "]" << endl;\n'
    test_code += '    return 0;\n'
    test_code += '}\n'

    return test_code


def generate_cpp_lca_harness(solution_code: str, func_name: str, test_cases: list) -> str:
    """Generate a C++ test harness for lowestCommonAncestor (needs to find nodes by value)."""
    import json

    test_code = CPP_HELPERS + '\n' + solution_code + '\n\n'
    test_code += 'int main() {\n'
    test_code += '    Solution sol;\n'
    test_code += '    cout << "[";\n'
    test_code += '    bool first = true;\n\n'

    for i, tc in enumerate(test_cases):
        inp = tc.get('input', {})
        expected = tc.get('expected')

        # Input format: {root: [...], p: int, q: int}
        root_arr = inp.get('root', [])
        p_val = inp.get('p', 0)
        q_val = inp.get('q', 0)

        test_code += f'    // Test case {i}\n'
        test_code += '    {\n'
        test_code += '        if (!first) cout << ",";\n'
        test_code += '        first = false;\n'
        test_code += '        try {\n'

        # Create the tree
        if root_arr:
            arr_vals = [str(x) + 'LL' if x is not None else 'TREE_NULL' for x in root_arr]
            arr_str = ','.join(arr_vals)
            test_code += f'            TreeNode* root = vectorToTree({{{arr_str}}});\n'
        else:
            test_code += '            TreeNode* root = nullptr;\n'

        # Find nodes by value
        test_code += f'            TreeNode* p = findNode(root, {p_val});\n'
        test_code += f'            TreeNode* q = findNode(root, {q_val});\n'

        test_code += f'            TreeNode* result = sol.{func_name}(root, p, q);\n'
        test_code += '            int got_val = result ? result->val : INT_MIN;\n'

        test_code += f'            int expected_val = {expected};\n'
        test_code += '            bool pass = got_val == expected_val;\n'
        test_code += '            cout << "{\\"pass\\":" << (pass ? "true" : "false") << ",\\"got\\":" << got_val << ",\\"expected\\":" << expected_val << "}";\n'
        test_code += '        } catch (exception& e) {\n'
        test_code += '            cout << "{\\"pass\\":false,\\"error\\":\\"" << e.what() << "\\"}";\n'
        test_code += '        } catch (...) {\n'
        test_code += '            cout << "{\\"pass\\":false,\\"error\\":\\"unknown error\\"}";\n'
        test_code += '        }\n'
        test_code += '    }\n\n'

    test_code += '    cout << "]" << endl;\n'
    test_code += '    return 0;\n'
    test_code += '}\n'

    return test_code


def generate_cpp_random_list_harness(solution_code: str, func_name: str, test_cases: list) -> str:
    """Generate a C++ test harness for copyRandomList (Node* with random pointer)."""
    import json

    test_code = CPP_HELPERS + '\n' + solution_code + '\n\n'
    test_code += 'int main() {\n'
    test_code += '    Solution sol;\n'
    test_code += '    cout << "[";\n'
    test_code += '    bool first = true;\n\n'

    for i, tc in enumerate(test_cases):
        inp = tc.get('input', [])
        expected = tc.get('expected', [])

        # Input format: [[[val, random_idx], ...]]
        arr = inp[0] if len(inp) > 0 else []

        test_code += f'    // Test case {i}\n'
        test_code += '    {\n'
        test_code += '        if (!first) cout << ",";\n'
        test_code += '        first = false;\n'
        test_code += '        try {\n'

        # Create the random list initialization
        if arr:
            pairs = []
            for item in arr:
                val = item[0]
                rand_idx = item[1] if item[1] is not None else -1
                pairs.append(f'{{{val},{rand_idx}}}')
            pairs_str = ','.join(pairs)
            test_code += f'            vector<pair<int, int>> arr = {{{pairs_str}}};\n'
            test_code += '            RandomNode* head = createRandomList(arr);\n'
        else:
            test_code += '            RandomNode* head = nullptr;\n'

        test_code += f'            RandomNode* result = sol.{func_name}(head);\n'
        test_code += '            string got = randomListToJson(result);\n'

        expected_json = json.dumps(expected, separators=(',', ':'))
        test_code += f'            string expected = R"JSON({expected_json})JSON";\n'
        test_code += '            bool pass = got == expected;\n'
        test_code += '            cout << "{\\"pass\\":" << (pass ? "true" : "false") << ",\\"got\\":" << got << ",\\"expected\\":" << expected << "}";\n'
        test_code += '        } catch (exception& e) {\n'
        test_code += '            cout << "{\\"pass\\":false,\\"error\\":\\"" << e.what() << "\\"}";\n'
        test_code += '        } catch (...) {\n'
        test_code += '            cout << "{\\"pass\\":false,\\"error\\":\\"unknown error\\"}";\n'
        test_code += '        }\n'
        test_code += '    }\n\n'

    test_code += '    cout << "]" << endl;\n'
    test_code += '    return 0;\n'
    test_code += '}\n'

    return test_code


def run_cpp_compiled(harness_code: str, timeout: int = 60) -> tuple:
    """Compile and run C++ code, return (stdout, stderr, returncode)."""
    with tempfile.TemporaryDirectory() as tmpdir:
        src_file = os.path.join(tmpdir, 'test.cpp')
        exe_file = os.path.join(tmpdir, 'test')

        with open(src_file, 'w') as f:
            f.write(harness_code)

        # Compile
        compile_result = subprocess.run(
            ['g++', '-std=c++17', '-O2', '-o', exe_file, src_file],
            capture_output=True,
            text=True,
            timeout=30
        )

        if compile_result.returncode != 0:
            return "", f"Compile error: {compile_result.stderr[:500]}", 1

        # Run
        try:
            run_result = subprocess.run(
                [exe_file],
                capture_output=True,
                text=True,
                timeout=timeout
            )
            return run_result.stdout, run_result.stderr, run_result.returncode
        except subprocess.TimeoutExpired:
            return "", "Timeout", -1


def test_cpp(problem_id: str, solution_file: Path) -> TestResult:
    """Test a C++ solution."""
    start = time.time()

    problem_info = get_problem_info(problem_id)
    test_cases = get_test_cases(problem_id)

    if not problem_info or not test_cases:
        return TestResult(problem_id, 'cpp', error="Missing files")

    with open(solution_file) as f:
        solution_code = f.read()

    func_name = problem_info.get('function_name', '')

    # Handle special problem types
    if func_name in CPP_CLASS_FUNCS:
        harness = generate_cpp_class_harness(solution_code, CPP_CLASS_FUNCS[func_name], test_cases)
    elif func_name in CPP_CYCLE_FUNCS:
        harness = generate_cpp_cycle_harness(solution_code, func_name, test_cases)
    elif func_name in CPP_INPLACE_LIST_FUNCS:
        harness = generate_cpp_inplace_list_harness(solution_code, func_name, test_cases)
    elif func_name in CPP_RANDOM_LIST_FUNCS:
        harness = generate_cpp_random_list_harness(solution_code, func_name, test_cases)
    elif func_name in CPP_LCA_FUNCS:
        harness = generate_cpp_lca_harness(solution_code, func_name, test_cases)
    else:
        is_order_independent = func_name in CPP_ORDER_INDEPENDENT
        is_multi_answer = func_name in CPP_MULTI_ANSWER
        harness = generate_cpp_harness(solution_code, func_name, test_cases, is_order_independent, is_multi_answer)

    if not harness:
        return TestResult(problem_id, 'cpp', error=f"Could not parse function {func_name}")

    stdout, stderr, rc = run_cpp_compiled(harness)

    if rc != 0 or not stdout.strip():
        return TestResult(problem_id, 'cpp', error=stderr[:500] or "Unknown error")

    try:
        results = json.loads(stdout.strip())
        passed = sum(1 for r in results if r.get('pass'))
        total = len(results)
        failures = [r for r in results if not r.get('pass')][:3]
        duration = (time.time() - start) * 1000
        return TestResult(problem_id, 'cpp', passed, total, failures, None, duration)
    except json.JSONDecodeError as e:
        return TestResult(problem_id, 'cpp', error=f"Parse error: {stdout[:200]}")


def test_c(problem_id: str, solution_file: Path) -> TestResult:
    """Test a C solution (not yet implemented)."""
    return TestResult(problem_id, 'c', error="C testing not yet implemented")


# ============================================================================
# Discovery
# ============================================================================

def discover_solutions(languages: list) -> dict:
    """Find all solution files grouped by language."""
    ext_map = {'python': '.py', 'javascript': '.js', 'cpp': '.cpp', 'c': '.c'}
    solutions = {}

    for lang in languages:
        ext = ext_map.get(lang)
        if ext:
            files = sorted(SOLUTIONS_DIR.glob(f"*{ext}"))
            solutions[lang] = [(f.stem.split('_')[0], f) for f in files]

    return solutions


# ============================================================================
# Worker function for parallel execution
# ============================================================================

def run_test_worker(args: tuple) -> TestResult:
    """Worker function for parallel test execution."""
    problem_id, solution_file, language = args

    runners = {
        'python': test_python,
        'javascript': test_javascript,
        'cpp': test_cpp,
        'c': test_c
    }

    runner = runners.get(language)
    if runner:
        return runner(problem_id, solution_file)
    return TestResult(problem_id, language, error="Unknown language")


# ============================================================================
# Display
# ============================================================================

def print_simple_progress(results: dict, totals: dict):
    """Simple progress output without rich."""
    for lang in results:
        passed = sum(1 for r in results[lang] if r.success)
        total = totals[lang]
        tests = sum(r.total for r in results[lang])
        status = "PASS" if passed == total else "FAIL"
        print(f"{lang:12} {passed:3}/{total:3} solutions  {tests:5} tests  {status}")


def print_rich_results(results: dict, totals: dict, duration: float):
    """Print results using rich library."""
    console = Console()

    # Summary table
    table = Table(title="Test Results", show_header=True, header_style="bold")
    table.add_column("Language", style="cyan")
    table.add_column("Solutions", justify="right")
    table.add_column("Tests", justify="right")
    table.add_column("Status", justify="center")

    total_solutions = 0
    passed_solutions = 0
    total_tests = 0

    for lang in ['javascript', 'python', 'cpp', 'c']:
        if lang not in results:
            continue

        passed = sum(1 for r in results[lang] if r.success)
        total = totals[lang]
        tests = sum(r.total for r in results[lang])

        total_solutions += total
        passed_solutions += passed
        total_tests += tests

        status = "[green]PASS[/green]" if passed == total else f"[red]{passed}/{total}[/red]"
        table.add_row(lang.capitalize(), f"{passed}/{total}", str(tests), status)

    console.print(table)
    console.print(f"\nTotal: {passed_solutions}/{total_solutions} solutions, {total_tests} tests in {duration:.1f}s")

    # Show failures
    for lang, lang_results in results.items():
        failures = [r for r in lang_results if not r.success and r.error is None]
        if failures:
            console.print(f"\n[bold red]{lang.capitalize()} Failures:[/bold red]")
            for r in failures[:5]:
                console.print(f"  {r.problem_id}: {r.passed}/{r.total}")
                for f in r.failures[:2]:
                    if 'error' in f:
                        console.print(f"    Error: {f['error'][:60]}")
                    else:
                        console.print(f"    Input: {json.dumps(f.get('input'))[:50]}")
                        console.print(f"    Expected: {f.get('expected')}, Got: {f.get('got')}")


# ============================================================================
# Main
# ============================================================================

def main():
    parser = argparse.ArgumentParser(description='Unified parallel test runner for LeetCode solutions')
    parser.add_argument('--lang', '-l', type=str, default='python,javascript',
                       help='Languages to test (comma-separated: python,javascript,cpp,c)')
    parser.add_argument('--problem', '-p', type=str, help='Test specific problem ID (e.g., 001)')
    parser.add_argument('--workers', '-w', type=int, default=8, help='Number of parallel workers')
    parser.add_argument('--json', '-j', type=str, help='Export results to JSON file')
    parser.add_argument('--verbose', '-v', action='store_true', help='Show all failures')
    args = parser.parse_args()

    languages = [l.strip() for l in args.lang.split(',')]
    solutions = discover_solutions(languages)

    if args.problem:
        solutions = {lang: [(pid, f) for pid, f in files if pid == args.problem]
                    for lang, files in solutions.items()}

    # Build task list
    tasks = []
    totals = {}
    for lang, files in solutions.items():
        totals[lang] = len(files)
        for problem_id, solution_file in files:
            tasks.append((problem_id, solution_file, lang))

    if not tasks:
        print("No solutions found")
        return

    print(f"Running {len(tasks)} tests across {len(languages)} language(s) with {args.workers} workers...\n")

    start_time = time.time()
    results = {lang: [] for lang in languages}

    # Run tests in parallel
    with ProcessPoolExecutor(max_workers=args.workers) as executor:
        futures = {executor.submit(run_test_worker, task): task for task in tasks}

        completed = 0
        for future in as_completed(futures):
            result = future.result()
            results[result.language].append(result)
            completed += 1

            # Simple progress
            if not RICH_AVAILABLE:
                status = "PASS" if result.success else "FAIL"
                print(f"[{completed}/{len(tasks)}] {result.language}:{result.problem_id} {result.passed}/{result.total} {status}")

    duration = time.time() - start_time

    # Display results
    print()
    if RICH_AVAILABLE:
        print_rich_results(results, totals, duration)
    else:
        print("=" * 60)
        print_simple_progress(results, totals)
        print("=" * 60)
        print(f"Completed in {duration:.1f}s")

    # Export JSON if requested
    if args.json:
        export = {
            lang: [{'problem_id': r.problem_id, 'passed': r.passed, 'total': r.total,
                   'success': r.success, 'error': r.error} for r in lang_results]
            for lang, lang_results in results.items()
        }
        with open(args.json, 'w') as f:
            json.dump(export, f, indent=2)
        print(f"\nResults exported to {args.json}")

    # Exit with failure if any tests failed
    all_passed = all(r.success for lang_results in results.values() for r in lang_results)
    sys.exit(0 if all_passed else 1)


if __name__ == '__main__':
    main()
