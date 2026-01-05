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
# C++ Runner (Placeholder - needs compile step)
# ============================================================================

def test_cpp(problem_id: str, solution_file: Path) -> TestResult:
    """Test a C++ solution (not yet implemented)."""
    return TestResult(problem_id, 'cpp', error="C++ testing not yet implemented")


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
