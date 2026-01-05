#!/usr/bin/env python3
"""Analyze JS solutions against comprehensive testcases and document issues."""

import json
import subprocess
import os
import glob
from pathlib import Path

PROJECT_DIR = Path(__file__).parent.parent
SOLUTIONS_DIR = PROJECT_DIR / "scripts" / "test_solutions"
TESTCASES_DIR = PROJECT_DIR / "testcases"
PROBLEMS_DIR = PROJECT_DIR / "problems"

def get_problem_info(padded_id):
    """Get problem metadata."""
    problem_files = list(PROBLEMS_DIR.glob(f"{padded_id}_*.json"))
    if not problem_files:
        return None
    with open(problem_files[0]) as f:
        return json.load(f)

def get_testcases(padded_id):
    """Get comprehensive test cases."""
    tc_files = list(TESTCASES_DIR.glob(f"{padded_id}_*.json"))
    if not tc_files:
        # Fall back to problems dir
        problem_files = list(PROBLEMS_DIR.glob(f"{padded_id}_*.json"))
        if problem_files:
            with open(problem_files[0]) as f:
                p = json.load(f)
                return p.get('test_cases', [])
        return []

    with open(tc_files[0]) as f:
        tc = json.load(f)
        return tc.get('run_tests', []) + tc.get('submit_tests', [])

def run_js_test(solution_file, testcases, func_name):
    """Run JS solution against test cases and return results."""
    with open(solution_file) as f:
        solution = f.read()

    # Create test script
    test_code = f"""
{solution}

const testCases = {json.dumps(testcases)};

// Helper functions
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

const linkedListFuncs = ['addTwoNumbers', 'removeNthFromEnd', 'mergeTwoLists',
    'reverseKGroup', 'reverseList', 'reorderList'];
const treeFuncs = ['invertTree', 'maxDepth', 'diameterOfBinaryTree', 'isBalanced', 'isSameTree',
    'isSubtree', 'levelOrder', 'rightSideView', 'isValidBST', 'kthSmallest', 'maxPathSum',
    'goodNodes', 'buildTree'];
const inPlaceFuncs = ['rotate', 'setZeroes', 'wallsAndGates', 'reorderList', 'solve'];
const skipFuncs = ['LRUCache', 'MinStack', 'Trie', 'trie', 'WordDictionary', 'wordDictionary',
    'MedianFinder', 'medianFinder', 'Twitter', 'KthLargest', 'TimeMap', 'DetectSquares',
    'Codec', 'codec', 'encodeDecode', 'cloneGraph', 'copyRandomList', 'hasCycle',
    'detectCycle', 'mergeKLists', 'lowestCommonAncestor'];

const isLinkedList = linkedListFuncs.includes('{func_name}');
const isTree = treeFuncs.includes('{func_name}');
const isInPlace = inPlaceFuncs.includes('{func_name}');
const isSkip = skipFuncs.includes('{func_name}');

if (isSkip) {{
    console.log(JSON.stringify({{ skipped: true }}));
    process.exit(0);
}}

const results = [];
for (let i = 0; i < testCases.length; i++) {{
    const tc = testCases[i];
    try {{
        // Handle both array and object input formats
        let inputs;
        if (Array.isArray(tc.input)) {{
            inputs = [...tc.input];
        }} else if (typeof tc.input === 'object' && tc.input !== null) {{
            // Object format: extract values in order
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
        // Convert tree result to array
        if (result && typeof result === 'object' && 'val' in result && ('left' in result || 'right' in result)) {{
            result = treeToArray(result);
        }}
        // Handle null result for empty array expected
        if (result === null && Array.isArray(tc.expected) && tc.expected.length === 0) {{
            result = [];
        }}

        results.push({{
            index: i,
            input: tc.input,
            expected: tc.expected,
            got: result,
            pass: JSON.stringify(result) === JSON.stringify(tc.expected)
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
"""

    # Write to temp file and run
    import tempfile
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
            return [{'error': result.stderr, 'pass': False}]
    except subprocess.TimeoutExpired:
        os.unlink(temp_file)
        return [{'error': 'timeout', 'pass': False}]
    except Exception as e:
        os.unlink(temp_file)
        return [{'error': str(e), 'pass': False}]

def analyze_all():
    """Analyze all JS solutions."""
    report = {
        'summary': {'total': 0, 'passed': 0, 'failed': 0, 'skipped': 0},
        'problems': []
    }

    solution_files = sorted(SOLUTIONS_DIR.glob("*.js"))

    for sol_file in solution_files:
        padded_id = sol_file.name.split('_')[0]
        problem_info = get_problem_info(padded_id)

        if not problem_info:
            continue

        func_name = problem_info.get('function_name', '')
        testcases = get_testcases(padded_id)

        if not testcases:
            continue

        report['summary']['total'] += 1

        print(f"Testing {padded_id} ({func_name})...", end=' ', flush=True)

        results = run_js_test(sol_file, testcases, func_name)

        if not results or (isinstance(results, list) and len(results) == 0):
            report['summary']['skipped'] += 1
            print("NO RESULTS")
            continue

        if isinstance(results, dict):
            if results.get('skipped'):
                report['summary']['skipped'] += 1
                print("SKIPPED")
                continue
            results = [results]

        if len(results) == 1 and results[0].get('skipped'):
            report['summary']['skipped'] += 1
            print("SKIPPED")
            continue

        passed = sum(1 for r in results if r.get('pass', False))
        failed = len(results) - passed

        problem_report = {
            'id': padded_id,
            'name': problem_info.get('title', ''),
            'func': func_name,
            'total_cases': len(testcases),
            'passed': passed,
            'failed': failed,
            'failures': []
        }

        if failed == 0:
            report['summary']['passed'] += 1
            print(f"PASS ({passed}/{len(results)})")
        else:
            report['summary']['failed'] += 1
            print(f"FAIL ({passed}/{len(results)})")

            # Record first 5 failures
            for r in results:
                if not r.get('pass', False) and len(problem_report['failures']) < 5:
                    problem_report['failures'].append({
                        'input': r.get('input'),
                        'expected': r.get('expected'),
                        'got': r.get('got'),
                        'error': r.get('error')
                    })

        report['problems'].append(problem_report)

    return report

if __name__ == '__main__':
    report = analyze_all()

    # Write report
    with open(PROJECT_DIR / 'scripts' / 'test_report.json', 'w') as f:
        json.dump(report, f, indent=2)

    print("\n" + "="*60)
    print("SUMMARY")
    print("="*60)
    print(f"Total: {report['summary']['total']}")
    print(f"Passed: {report['summary']['passed']}")
    print(f"Failed: {report['summary']['failed']}")
    print(f"Skipped: {report['summary']['skipped']}")

    print("\n" + "="*60)
    print("FAILED PROBLEMS")
    print("="*60)
    for p in report['problems']:
        if p['failed'] > 0:
            print(f"\n{p['id']} {p['name']} ({p['func']})")
            print(f"  {p['passed']}/{p['total_cases']} passed")
            for f in p['failures'][:3]:
                if f.get('error'):
                    print(f"  ERROR: {f['error'][:100]}")
                else:
                    print(f"  input: {str(f['input'])[:60]}")
                    print(f"  expected: {str(f['expected'])[:40]}, got: {str(f['got'])[:40]}")
