#!/usr/bin/env python3
"""Fix test data issues and validate all solutions."""

import json
import subprocess
import tempfile
import os
from pathlib import Path

PROJECT_DIR = Path(__file__).parent.parent
SOLUTIONS_DIR = PROJECT_DIR / "scripts" / "test_solutions"
TESTCASES_DIR = PROJECT_DIR / "testcases"
PROBLEMS_DIR = PROJECT_DIR / "problems"

def get_problem_info(padded_id):
    problem_files = list(PROBLEMS_DIR.glob(f"{padded_id}_*.json"))
    if not problem_files:
        return None
    with open(problem_files[0]) as f:
        return json.load(f)

def get_testcases_file(padded_id):
    tc_files = list(TESTCASES_DIR.glob(f"{padded_id}_*.json"))
    return tc_files[0] if tc_files else None

def run_class_test(solution_file, testcases, class_name):
    """Run class-based JS solution against test cases."""
    with open(solution_file) as f:
        solution = f.read()

    test_code = f'''
{solution}

const testCases = {json.dumps(testcases)};
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

    return run_js_code(test_code)


def run_codec_test(solution_file, testcases):
    """Run serialize/deserialize test for Codec."""
    with open(solution_file) as f:
        solution = f.read()

    test_code = f'''
{solution}

const testCases = {json.dumps(testcases)};

const arrayToTree = (arr) => {{
    if (!arr || arr.length === 0 || arr[0] === null) return null;
    const root = {{ val: arr[0], left: null, right: null }};
    const queue = [root];
    let i = 1;
    while (queue.length > 0 && i < arr.length) {{
        const node = queue.shift();
        if (i < arr.length && arr[i] !== null) {{
            node.left = {{ val: arr[i], left: null, right: null }};
            queue.push(node.left);
        }}
        i++;
        if (i < arr.length && arr[i] !== null) {{
            node.right = {{ val: arr[i], left: null, right: null }};
            queue.push(node.right);
        }}
        i++;
    }}
    return root;
}};

const treeToArray = (root) => {{
    if (!root) return [];
    const result = [];
    const queue = [root];
    while (queue.length > 0) {{
        const node = queue.shift();
        if (node) {{
            result.push(node.val);
            queue.push(node.left);
            queue.push(node.right);
        }} else {{
            result.push(null);
        }}
    }}
    while (result.length > 0 && result[result.length - 1] === null) result.pop();
    return result;
}};

const results = [];

for (const tc of testCases) {{
    try {{
        const inputArr = Array.isArray(tc.input) ? tc.input[0] : tc.input;
        const tree = arrayToTree(inputArr);
        const serialized = serialize(tree);
        const deserialized = deserialize(serialized);
        const result = treeToArray(deserialized);

        const pass = JSON.stringify(result) === JSON.stringify(tc.expected);
        results.push({{ input: tc.input, expected: tc.expected, got: result, pass }});
    }} catch (e) {{
        results.push({{ input: tc.input, expected: tc.expected, error: e.message, pass: false }});
    }}
}}

console.log(JSON.stringify(results));
'''

    return run_js_code(test_code)


def run_encode_decode_test(solution_file, testcases):
    """Run encode/decode test for strings."""
    with open(solution_file) as f:
        solution = f.read()

    test_code = f'''
{solution}

const testCases = {json.dumps(testcases)};
const results = [];

for (const tc of testCases) {{
    try {{
        // Input format: [[["Hello", "World"]]] - triple nested
        let inputArr = tc.input;
        while (Array.isArray(inputArr) && inputArr.length === 1 && Array.isArray(inputArr[0])) {{
            inputArr = inputArr[0];
        }}
        const encoded = encode(inputArr);
        const decoded = decode(encoded);

        const pass = JSON.stringify(decoded) === JSON.stringify(tc.expected);
        results.push({{ input: tc.input, expected: tc.expected, got: decoded, pass }});
    }} catch (e) {{
        results.push({{ input: tc.input, expected: tc.expected, error: e.message, pass: false }});
    }}
}}

console.log(JSON.stringify(results));
'''

    return run_js_code(test_code)


def run_clone_graph_test(solution_file, testcases):
    """Run cloneGraph test with adjacency list input."""
    with open(solution_file) as f:
        solution = f.read()

    test_code = f'''
{solution}

const testCases = {json.dumps(testcases)};

const buildGraph = (adjList) => {{
    if (!adjList || adjList.length === 0) return null;
    const nodes = adjList.map((_, i) => ({{ val: i + 1, neighbors: [] }}));
    adjList.forEach((neighbors, i) => {{
        nodes[i].neighbors = neighbors.map(n => nodes[n - 1]);
    }});
    return nodes[0];
}};

const graphToAdj = (node) => {{
    if (!node) return [];
    const visited = new Map();
    const result = [];
    const queue = [node];
    visited.set(node.val, true);

    // BFS to find all nodes first
    const nodes = [];
    while (queue.length > 0) {{
        const curr = queue.shift();
        nodes.push(curr);
        for (const neighbor of curr.neighbors) {{
            if (!visited.has(neighbor.val)) {{
                visited.set(neighbor.val, true);
                queue.push(neighbor);
            }}
        }}
    }}

    // Sort by val and build adjacency list
    nodes.sort((a, b) => a.val - b.val);
    return nodes.map(n => n.neighbors.map(nb => nb.val).sort((a, b) => a - b));
}};

const results = [];

for (const tc of testCases) {{
    try {{
        const adjList = tc.input.adjList !== undefined ? tc.input.adjList : tc.input[0];
        const graph = buildGraph(adjList);
        const cloned = cloneGraph(graph);
        const result = graphToAdj(cloned);

        const pass = JSON.stringify(result) === JSON.stringify(tc.expected);
        results.push({{ input: tc.input, expected: tc.expected, got: result, pass }});
    }} catch (e) {{
        results.push({{ input: tc.input, expected: tc.expected, error: e.message, pass: false }});
    }}
}}

console.log(JSON.stringify(results));
'''

    return run_js_code(test_code)


def run_copy_random_list_test(solution_file, testcases):
    """Run copyRandomList test with [val, randomIndex] input."""
    with open(solution_file) as f:
        solution = f.read()

    test_code = f'''
{solution}

const testCases = {json.dumps(testcases)};

const buildList = (arr) => {{
    if (!arr || arr.length === 0) return null;
    const nodes = arr.map(([val]) => ({{ val, next: null, random: null }}));
    arr.forEach(([val, randomIdx], i) => {{
        if (i < arr.length - 1) nodes[i].next = nodes[i + 1];
        if (randomIdx !== null) nodes[i].random = nodes[randomIdx];
    }});
    return nodes[0];
}};

const listToArr = (head) => {{
    if (!head) return [];
    const result = [];
    const nodeToIdx = new Map();
    let curr = head;
    let idx = 0;
    while (curr) {{
        nodeToIdx.set(curr, idx++);
        curr = curr.next;
    }}
    curr = head;
    while (curr) {{
        const randomIdx = curr.random ? nodeToIdx.get(curr.random) : null;
        result.push([curr.val, randomIdx]);
        curr = curr.next;
    }}
    return result;
}};

const results = [];

for (const tc of testCases) {{
    try {{
        const inputArr = Array.isArray(tc.input[0]) ? tc.input[0] : tc.input;
        const list = buildList(inputArr);
        const copied = copyRandomList(list);
        const result = listToArr(copied);

        const pass = JSON.stringify(result) === JSON.stringify(tc.expected);
        results.push({{ input: tc.input, expected: tc.expected, got: result, pass }});
    }} catch (e) {{
        results.push({{ input: tc.input, expected: tc.expected, error: e.message, pass: false }});
    }}
}}

console.log(JSON.stringify(results));
'''

    return run_js_code(test_code)


def run_has_cycle_test(solution_file, testcases):
    """Run hasCycle test with cyclic linked list."""
    with open(solution_file) as f:
        solution = f.read()

    test_code = f'''
{solution}

const testCases = {json.dumps(testcases)};

const buildCyclicList = (arr, pos) => {{
    if (!arr || arr.length === 0) return null;
    const nodes = arr.map(v => ({{ val: v, next: null }}));
    for (let i = 0; i < nodes.length - 1; i++) nodes[i].next = nodes[i + 1];
    if (pos >= 0 && pos < nodes.length) nodes[nodes.length - 1].next = nodes[pos];
    return nodes[0];
}};

const results = [];

for (const tc of testCases) {{
    try {{
        const [arr, pos] = Array.isArray(tc.input) ? tc.input : Object.values(tc.input);
        const list = buildCyclicList(arr, pos);
        const result = hasCycle(list);

        const pass = result === tc.expected;
        results.push({{ input: tc.input, expected: tc.expected, got: result, pass }});
    }} catch (e) {{
        results.push({{ input: tc.input, expected: tc.expected, error: e.message, pass: false }});
    }}
}}

console.log(JSON.stringify(results));
'''

    return run_js_code(test_code)


def run_merge_k_lists_test(solution_file, testcases):
    """Run mergeKLists test with array of linked lists."""
    with open(solution_file) as f:
        solution = f.read()

    test_code = f'''
{solution}

const testCases = {json.dumps(testcases)};

const arrayToList = (arr) => {{
    if (!arr || arr.length === 0) return null;
    let head = {{ val: arr[0], next: null }};
    let curr = head;
    for (let i = 1; i < arr.length; i++) {{
        curr.next = {{ val: arr[i], next: null }};
        curr = curr.next;
    }}
    return head;
}};

const listToArray = (head) => {{
    const result = [];
    while (head) {{
        result.push(head.val);
        head = head.next;
    }}
    return result;
}};

const results = [];

for (const tc of testCases) {{
    try {{
        const listsArr = Array.isArray(tc.input[0]) ? tc.input[0] : tc.input;
        const lists = listsArr.map(arr => arrayToList(arr));
        const merged = mergeKLists(lists);
        const result = listToArray(merged);

        const pass = JSON.stringify(result) === JSON.stringify(tc.expected);
        results.push({{ input: tc.input, expected: tc.expected, got: result, pass }});
    }} catch (e) {{
        results.push({{ input: tc.input, expected: tc.expected, error: e.message, pass: false }});
    }}
}}

console.log(JSON.stringify(results));
'''

    return run_js_code(test_code)


def run_lca_test(solution_file, testcases):
    """Run lowestCommonAncestor test with tree and node values."""
    with open(solution_file) as f:
        solution = f.read()

    test_code = f'''
{solution}

const testCases = {json.dumps(testcases)};

const arrayToTree = (arr) => {{
    if (!arr || arr.length === 0 || arr[0] === null) return null;
    const root = {{ val: arr[0], left: null, right: null }};
    const queue = [root];
    let i = 1;
    while (queue.length > 0 && i < arr.length) {{
        const node = queue.shift();
        if (i < arr.length && arr[i] !== null) {{
            node.left = {{ val: arr[i], left: null, right: null }};
            queue.push(node.left);
        }}
        i++;
        if (i < arr.length && arr[i] !== null) {{
            node.right = {{ val: arr[i], left: null, right: null }};
            queue.push(node.right);
        }}
        i++;
    }}
    return root;
}};

const findNode = (root, val) => {{
    if (!root) return null;
    if (root.val === val) return root;
    return findNode(root.left, val) || findNode(root.right, val);
}};

const results = [];

for (const tc of testCases) {{
    try {{
        const {{ root: rootArr, p: pVal, q: qVal }} = tc.input;
        const root = arrayToTree(rootArr);
        const p = findNode(root, pVal);
        const q = findNode(root, qVal);
        const result = lowestCommonAncestor(root, p, q);
        const resultVal = result ? result.val : null;

        const pass = resultVal === tc.expected;
        results.push({{ input: tc.input, expected: tc.expected, got: resultVal, pass }});
    }} catch (e) {{
        results.push({{ input: tc.input, expected: tc.expected, error: e.message, pass: false }});
    }}
}}

console.log(JSON.stringify(results));
'''

    return run_js_code(test_code)


def run_js_code(test_code):
    """Execute JavaScript code and return parsed results."""
    with tempfile.NamedTemporaryFile(mode='w', suffix='.js', delete=False) as f:
        f.write(test_code)
        temp_file = f.name

    try:
        result = subprocess.run(
            ['bun', 'run', temp_file],
            capture_output=True,
            text=True,
            timeout=60
        )
        os.unlink(temp_file)

        if result.returncode == 0:
            return json.loads(result.stdout)
        else:
            return [{'error': result.stderr[:500], 'pass': False}]
    except subprocess.TimeoutExpired:
        os.unlink(temp_file)
        return [{'error': 'timeout', 'pass': False}]
    except Exception as e:
        os.unlink(temp_file)
        return [{'error': str(e), 'pass': False}]


def run_test(solution_file, testcases, func_name):
    """Run JS solution against test cases."""
    with open(solution_file) as f:
        solution = f.read()

    # Determine function type for special handling
    linked_list_funcs = ['addTwoNumbers', 'removeNthFromEnd', 'mergeTwoLists',
        'reverseKGroup', 'reverseList', 'reorderList']
    # Tree functions - these take tree input and may return tree output
    tree_funcs = ['invertTree', 'maxDepth', 'diameterOfBinaryTree', 'isBalanced', 'isSameTree',
        'isSubtree', 'levelOrder', 'rightSideView', 'isValidBST', 'kthSmallest', 'maxPathSum',
        'goodNodes']
    # buildTree takes arrays, returns tree - needs special output handling
    tree_output_funcs = ['buildTree']
    in_place_funcs = ['rotate', 'setZeroes', 'wallsAndGates', 'reorderList', 'solve']

    # Class-based problems (map function_name to class name)
    class_funcs = {
        'LRUCache': 'LRUCache', 'MinStack': 'MinStack',
        'Trie': 'Trie', 'trie': 'Trie',
        'WordDictionary': 'WordDictionary', 'wordDictionary': 'WordDictionary',
        'MedianFinder': 'MedianFinder', 'medianFinder': 'MedianFinder',
        'Twitter': 'Twitter', 'twitter': 'Twitter',
        'KthLargest': 'KthLargest', 'kthLargest': 'KthLargest',
        'TimeMap': 'TimeMap', 'timeMap': 'TimeMap',
        'DetectSquares': 'DetectSquares', 'detectSquares': 'DetectSquares'
    }

    # Special structure problems - route to specific handlers
    if func_name in class_funcs:
        return run_class_test(solution_file, testcases, class_funcs[func_name])
    if func_name in ['Codec', 'codec', 'serialize']:
        return run_codec_test(solution_file, testcases)
    if func_name in ['encodeDecode', 'encode']:
        return run_encode_decode_test(solution_file, testcases)
    if func_name == 'cloneGraph':
        return run_clone_graph_test(solution_file, testcases)
    if func_name == 'copyRandomList':
        return run_copy_random_list_test(solution_file, testcases)
    if func_name in ['hasCycle', 'detectCycle']:
        return run_has_cycle_test(solution_file, testcases)
    if func_name == 'mergeKLists':
        return run_merge_k_lists_test(solution_file, testcases)
    if func_name == 'lowestCommonAncestor':
        return run_lca_test(solution_file, testcases)

    # Problems where output order doesn't matter (set-like results)
    order_independent_funcs = ['subsets', 'subsetsWithDup', 'combinationSum', 'combinationSum2',
        'permute', 'permuteUnique', 'generateParenthesis', 'solveNQueens', 'partition',
        'groupAnagrams', 'findWords', 'threeSum', 'letterCombinations']
    # Problems that return floating point numbers
    float_funcs = ['myPow', 'findMedianSortedArrays']
    # Problems where expected is array of valid answers (any match is correct)
    multi_answer_funcs = ['longestPalindrome']

    is_linked_list = func_name in linked_list_funcs
    is_tree = func_name in tree_funcs
    is_tree_output = func_name in tree_output_funcs
    is_in_place = func_name in in_place_funcs
    is_order_independent = func_name in order_independent_funcs
    is_float = func_name in float_funcs
    is_multi_answer = func_name in multi_answer_funcs

    test_code = f'''
{solution}

const testCases = {json.dumps(testcases)};

const arrayToList = (arr) => {{
    if (!arr || arr.length === 0) return null;
    let head = {{ val: arr[0], next: null }};
    let curr = head;
    for (let i = 1; i < arr.length; i++) {{
        curr.next = {{ val: arr[i], next: null }};
        curr = curr.next;
    }}
    return head;
}};

const listToArray = (head) => {{
    const result = [];
    while (head) {{
        result.push(head.val);
        head = head.next;
    }}
    return result;
}};

const arrayToTree = (arr) => {{
    if (!arr || arr.length === 0 || arr[0] === null) return null;
    const root = {{ val: arr[0], left: null, right: null }};
    const queue = [root];
    let i = 1;
    while (queue.length > 0 && i < arr.length) {{
        const node = queue.shift();
        if (i < arr.length && arr[i] !== null) {{
            node.left = {{ val: arr[i], left: null, right: null }};
            queue.push(node.left);
        }}
        i++;
        if (i < arr.length && arr[i] !== null) {{
            node.right = {{ val: arr[i], left: null, right: null }};
            queue.push(node.right);
        }}
        i++;
    }}
    return root;
}};

const treeToArray = (root) => {{
    if (!root) return [];
    const result = [];
    const queue = [root];
    while (queue.length > 0) {{
        const node = queue.shift();
        if (node) {{
            result.push(node.val);
            queue.push(node.left);
            queue.push(node.right);
        }} else {{
            result.push(null);
        }}
    }}
    while (result.length > 0 && result[result.length - 1] === null) result.pop();
    return result;
}};

const isLinkedList = {str(is_linked_list).lower()};
const isTree = {str(is_tree).lower()};
const isInPlace = {str(is_in_place).lower()};
const isOrderIndependent = {str(is_order_independent).lower()};
const isFloat = {str(is_float).lower()};
const isTreeOutput = {str(is_tree_output).lower()};
const isMultiAnswer = {str(is_multi_answer).lower()};

// Order-independent comparison for set-like results
const sortNested = (arr) => {{
    if (!Array.isArray(arr)) return arr;
    return arr.map(sortNested).sort((a, b) => JSON.stringify(a).localeCompare(JSON.stringify(b)));
}};

const compareOrderIndependent = (a, b) => {{
    return JSON.stringify(sortNested(a)) === JSON.stringify(sortNested(b));
}};

// Float comparison with tolerance
const compareFloat = (a, b) => {{
    if (typeof a === 'number' && typeof b === 'number') {{
        return Math.abs(a - b) < 1e-5;
    }}
    return a === b;
}};

const results = [];
for (let i = 0; i < testCases.length; i++) {{
    const tc = testCases[i];
    try {{
        let inputs;
        if (Array.isArray(tc.input)) {{
            inputs = [...tc.input];
        }} else if (typeof tc.input === 'object' && tc.input !== null) {{
            inputs = Object.values(tc.input);
        }} else {{
            inputs = [tc.input];
        }}

        if (isLinkedList) {{
            inputs = inputs.map(inp => Array.isArray(inp) ? arrayToList(inp) : inp);
        }}
        if (isTree) {{
            inputs = inputs.map(inp => Array.isArray(inp) ? arrayToTree(inp) : inp);
        }}

        let result;
        if (isInPlace) {{
            {func_name}(...inputs);
            result = inputs[0];
        }} else {{
            result = {func_name}(...inputs);
        }}

        // Convert linked list result to array
        if (result && typeof result === 'object' && 'val' in result && 'next' in result) {{
            result = listToArray(result);
        }}
        // Convert tree result to array (check for tree node structure)
        else if (result && typeof result === 'object' && 'val' in result && ('left' in result || 'right' in result)) {{
            result = treeToArray(result);
        }}
        // For tree output functions, explicitly convert (handles edge case where root has no left/right yet)
        else if (isTreeOutput && result && typeof result === 'object' && !Array.isArray(result)) {{
            result = treeToArray(result);
        }}
        if (result === null && Array.isArray(tc.expected) && tc.expected.length === 0) {{
            result = [];
        }}

        let pass;
        if (isMultiAnswer && Array.isArray(tc.expected)) {{
            // Expected is array of valid answers - check if result matches any
            pass = tc.expected.includes(result);
        }} else if (isOrderIndependent) {{
            pass = compareOrderIndependent(result, tc.expected);
        }} else if (isFloat) {{
            pass = compareFloat(result, tc.expected);
        }} else {{
            pass = JSON.stringify(result) === JSON.stringify(tc.expected);
        }}
        results.push({{
            index: i,
            input: tc.input,
            expected: tc.expected,
            got: result,
            pass: pass
        }});
    }} catch (e) {{
        results.push({{
            index: i,
            input: tc.input,
            expected: tc.expected,
            error: e.message,
            pass: false
        }});
    }}
}}

console.log(JSON.stringify(results));
'''

    with tempfile.NamedTemporaryFile(mode='w', suffix='.js', delete=False) as f:
        f.write(test_code)
        temp_file = f.name

    try:
        result = subprocess.run(
            ['bun', 'run', temp_file],
            capture_output=True,
            text=True,
            timeout=60
        )
        os.unlink(temp_file)

        if result.returncode == 0:
            return json.loads(result.stdout)
        else:
            return [{'error': result.stderr[:500], 'pass': False}]
    except subprocess.TimeoutExpired:
        os.unlink(temp_file)
        return [{'error': 'timeout', 'pass': False}]
    except Exception as e:
        os.unlink(temp_file)
        return [{'error': str(e), 'pass': False}]


def main():
    solution_files = sorted(SOLUTIONS_DIR.glob("*.js"))

    results = {
        'passed': [],
        'failed': [],
        'skipped': [],
        'total_tests': 0,
        'total_passed': 0
    }

    for sol_file in solution_files:
        padded_id = sol_file.name.split('_')[0]
        problem_info = get_problem_info(padded_id)

        if not problem_info:
            continue

        func_name = problem_info.get('function_name', '')
        tc_file = get_testcases_file(padded_id)

        if not tc_file:
            continue

        with open(tc_file) as f:
            tc_data = json.load(f)
        testcases = tc_data.get('run_tests', []) + tc_data.get('submit_tests', [])

        if not testcases:
            continue

        print(f"Testing {padded_id} ({func_name})...", end=' ', flush=True)

        test_results = run_test(sol_file, testcases, func_name)

        if isinstance(test_results, dict) and test_results.get('skipped'):
            results['skipped'].append(padded_id)
            print("SKIPPED")
            continue

        if not test_results:
            results['skipped'].append(padded_id)
            print("NO RESULTS")
            continue

        passed = sum(1 for r in test_results if r.get('pass', False))
        total = len(test_results)

        # Track totals
        results['total_tests'] += total
        results['total_passed'] += passed

        if passed == total:
            results['passed'].append({'id': padded_id, 'tests': total})
            print(f"PASS ({passed}/{total})")
        else:
            failures = [r for r in test_results if not r.get('pass', False)][:3]
            results['failed'].append({
                'id': padded_id,
                'func': func_name,
                'passed': passed,
                'total': total,
                'failures': failures
            })
            print(f"FAIL ({passed}/{total})")

    print("\n" + "="*60)
    print("SUMMARY")
    print("="*60)
    print(f"Problems: {len(results['passed'])} passed, {len(results['failed'])} failed, {len(results['skipped'])} skipped")
    print(f"Test cases: {results['total_passed']}/{results['total_tests']} passed ({100*results['total_passed']//results['total_tests'] if results['total_tests'] else 0}%)")

    if results['failed']:
        print("\n" + "="*60)
        print("FAILURES")
        print("="*60)
        for f in results['failed']:
            print(f"\n{f['id']} ({f['func']}): {f['passed']}/{f['total']}")
            for fail in f['failures']:
                if fail.get('error'):
                    print(f"  ERROR: {fail['error'][:80]}")
                else:
                    inp_str = str(fail.get('input', ''))[:50]
                    exp_str = str(fail.get('expected', ''))[:30]
                    got_str = str(fail.get('got', ''))[:30]
                    print(f"  in: {inp_str}")
                    print(f"  exp: {exp_str}, got: {got_str}")

    return results

if __name__ == '__main__':
    main()
