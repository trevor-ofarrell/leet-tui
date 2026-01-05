#!/usr/bin/env python3
"""Generate comprehensive edge cases for all LeetCode problems."""

import json
import subprocess
import tempfile
import os
from pathlib import Path

PROJECT_DIR = Path(__file__).parent.parent
SOLUTIONS_DIR = PROJECT_DIR / "scripts" / "test_solutions"
TESTCASES_DIR = PROJECT_DIR / "testcases"
PROBLEMS_DIR = PROJECT_DIR / "problems"

# Edge case templates by input type
ARRAY_EDGE_CASES = [
    [],                          # empty
    [0],                         # single zero
    [1],                         # single positive
    [-1],                        # single negative
    [1, 2],                      # two elements
    [2, 1],                      # two elements reversed
    [1, 1],                      # two same
    [0, 0, 0],                   # all zeros
    [1, 1, 1, 1, 1],            # all same positive
    [-1, -1, -1],               # all same negative
    [1, 2, 3, 4, 5],            # sorted ascending
    [5, 4, 3, 2, 1],            # sorted descending
    [-5, -3, -1, 0, 2, 4],      # mixed with negatives sorted
    [1, -1, 2, -2, 0],          # alternating signs
    [100, 200, 300],            # larger values
    [-100, -200, -300],         # larger negatives
    [0, 1, 0, 1, 0],            # binary pattern
    [1, 2, 1, 2, 1, 2],         # repeating pattern
]

STRING_EDGE_CASES = [
    "",                          # empty
    "a",                         # single char
    "aa",                        # two same
    "ab",                        # two different
    "aaa",                       # all same
    "abc",                       # all different
    "aba",                       # palindrome odd
    "abba",                      # palindrome even
    "abcba",                     # palindrome longer
    "aaaaaa",                    # long same
    "abcdef",                    # ascending
    "zyxwvu",                    # descending
    "AaBbCc",                    # mixed case
    "a b c",                     # with spaces
    "   ",                       # only spaces
    "12345",                     # digits
    "a1b2c3",                    # alphanumeric
]

MATRIX_EDGE_CASES = [
    [],                          # empty
    [[]],                        # single empty row
    [[1]],                       # 1x1
    [[1, 2]],                    # 1x2
    [[1], [2]],                  # 2x1
    [[1, 2], [3, 4]],           # 2x2
    [[0, 0], [0, 0]],           # all zeros
    [[1, 1], [1, 1]],           # all ones
    [[1, 2, 3], [4, 5, 6], [7, 8, 9]],  # 3x3
]

TREE_EDGE_CASES = [
    [],                          # null/empty
    [1],                         # single node
    [1, 2],                      # root with left
    [1, None, 2],               # root with right
    [1, 2, 3],                  # complete 2 levels
    [1, 2, 3, 4, 5],            # partial 3 levels
    [1, 2, 3, 4, 5, 6, 7],      # complete 3 levels
    [1, None, 2, None, 3],      # right skewed
    [1, 2, None, 3, None, 4],   # left skewed
    [5, 3, 7, 2, 4, 6, 8],      # BST
    [1, 1, 1, 1, 1],            # all same values
    [-1, -2, -3],               # negative values
    [0],                         # zero root
    [1, 2, 2, 3, 4, 4, 3],      # symmetric
]

LINKEDLIST_EDGE_CASES = [
    [],                          # empty/null
    [1],                         # single node
    [1, 2],                      # two nodes
    [1, 1],                      # two same
    [1, 2, 3],                  # three nodes
    [1, 2, 3, 4, 5],            # longer
    [5, 4, 3, 2, 1],            # descending
    [1, 1, 1, 1],               # all same
    [0],                         # zero
    [-1, -2, -3],               # negatives
]

GRAPH_EDGE_CASES = [
    [],                          # empty
    [[]],                        # single node no edges
    [[2], [1]],                  # two nodes connected
    [[2, 3], [1, 3], [1, 2]],   # triangle
]

INTERVAL_EDGE_CASES = [
    [],                          # empty
    [[1, 2]],                    # single interval
    [[1, 2], [3, 4]],           # non-overlapping
    [[1, 3], [2, 4]],           # overlapping
    [[1, 4], [2, 3]],           # nested
    [[1, 2], [2, 3]],           # touching
    [[1, 1]],                    # point interval
    [[1, 5], [2, 3], [4, 6]],   # multiple overlaps
]

# Problem type detection and edge case mapping
def detect_problem_type(problem_info, existing_tests):
    """Detect the problem type from problem info and existing tests."""
    func_name = problem_info.get('function_name', '')

    # Check existing test structure
    if existing_tests:
        sample = existing_tests[0].get('input', {})
        if isinstance(sample, dict):
            keys = list(sample.keys())
        elif isinstance(sample, list):
            keys = []
            sample = {'arr': sample[0]} if sample else {}
        else:
            keys = []
            sample = {}
    else:
        return 'unknown', {}

    # Detect by parameter names
    param_types = {}
    for key in keys:
        val = sample.get(key)
        if key in ['root', 'tree', 'p', 'q']:
            param_types[key] = 'tree'
        elif key in ['head', 'list', 'l1', 'l2']:
            param_types[key] = 'linkedlist'
        elif key in ['graph', 'adjList', 'edges']:
            param_types[key] = 'graph'
        elif key in ['intervals', 'ranges']:
            param_types[key] = 'intervals'
        elif key in ['matrix', 'grid', 'board', 'image']:
            param_types[key] = 'matrix'
        elif key in ['s', 't', 'word', 'pattern', 'str', 'text']:
            param_types[key] = 'string'
        elif key in ['nums', 'arr', 'numbers', 'candidates', 'heights', 'prices', 'coins', 'weights', 'values']:
            param_types[key] = 'array'
        elif key in ['n', 'k', 'target', 'val', 'x', 'capacity']:
            param_types[key] = 'integer'
        elif isinstance(val, list) and val and isinstance(val[0], list):
            param_types[key] = 'matrix'
        elif isinstance(val, list):
            param_types[key] = 'array'
        elif isinstance(val, str):
            param_types[key] = 'string'
        elif isinstance(val, (int, float)):
            param_types[key] = 'integer'

    return param_types


def generate_edge_cases_for_problem(problem_id, problem_info, existing_tests):
    """Generate edge cases for a specific problem."""
    param_types = detect_problem_type(problem_info, existing_tests)

    if not param_types:
        return []

    # Get existing input structure
    if not existing_tests:
        return []

    sample_input = existing_tests[0].get('input', {})
    if isinstance(sample_input, list):
        # Convert array format to dict format
        sample_input = {f'arg{i}': v for i, v in enumerate(sample_input)}

    edge_cases = []

    # Generate combinations based on param types
    keys = list(param_types.keys())

    if len(keys) == 1:
        key = keys[0]
        ptype = param_types[key]
        cases = get_edge_cases_for_type(ptype)
        for case in cases:
            edge_cases.append({key: case})

    elif len(keys) == 2:
        # Two parameters - generate combinations
        key1, key2 = keys[0], keys[1]
        type1, type2 = param_types[key1], param_types[key2]

        cases1 = get_edge_cases_for_type(type1)[:5]  # Limit combinations
        cases2 = get_edge_cases_for_type(type2)[:5]

        for c1 in cases1:
            for c2 in cases2:
                edge_cases.append({key1: c1, key2: c2})

    else:
        # Multiple parameters - use sample structure with edge values
        for ptype_cases in [ARRAY_EDGE_CASES[:3], STRING_EDGE_CASES[:3]]:
            for case in ptype_cases:
                new_input = dict(sample_input)
                for key, ptype in param_types.items():
                    if ptype == 'array' and isinstance(case, list):
                        new_input[key] = case
                    elif ptype == 'string' and isinstance(case, str):
                        new_input[key] = case
                edge_cases.append(new_input)

    return edge_cases[:20]  # Limit to 20 edge cases per problem


def get_edge_cases_for_type(ptype):
    """Get edge cases for a parameter type."""
    if ptype == 'array':
        return ARRAY_EDGE_CASES
    elif ptype == 'string':
        return STRING_EDGE_CASES
    elif ptype == 'matrix':
        return MATRIX_EDGE_CASES
    elif ptype == 'tree':
        return TREE_EDGE_CASES
    elif ptype == 'linkedlist':
        return LINKEDLIST_EDGE_CASES
    elif ptype == 'graph':
        return GRAPH_EDGE_CASES
    elif ptype == 'intervals':
        return INTERVAL_EDGE_CASES
    elif ptype == 'integer':
        return [0, 1, -1, 2, 10, 100, -100]
    else:
        return []


def compute_expected_value(solution_file, func_name, input_case, problem_info):
    """Run solution to compute expected value for an input."""
    with open(solution_file) as f:
        solution = f.read()

    # Determine if we need special input/output handling
    tree_input_funcs = ['invertTree', 'maxDepth', 'diameterOfBinaryTree', 'isBalanced',
                        'isSameTree', 'isSubtree', 'levelOrder', 'rightSideView',
                        'isValidBST', 'kthSmallest', 'maxPathSum', 'goodNodes']
    tree_output_funcs = ['invertTree', 'buildTree']
    linkedlist_funcs = ['addTwoNumbers', 'removeNthFromEnd', 'mergeTwoLists',
                        'reverseKGroup', 'reverseList', 'reorderList', 'mergeKLists']

    is_tree_input = func_name in tree_input_funcs
    is_tree_output = func_name in tree_output_funcs
    is_linkedlist = func_name in linkedlist_funcs

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
'''

    test_code = solution + helpers + f'''
try {{
    let inputs = {json.dumps(list(input_case.values()) if isinstance(input_case, dict) else input_case)};

    {"inputs = inputs.map(inp => Array.isArray(inp) ? arrayToTree(inp) : inp);" if is_tree_input else ""}
    {"inputs = inputs.map(inp => Array.isArray(inp) ? arrayToList(inp) : inp);" if is_linkedlist else ""}

    let result = {func_name}(...inputs);

    if (result && typeof result === 'object' && 'val' in result && 'next' in result) {{
        result = listToArray(result);
    }}
    if (result && typeof result === 'object' && 'val' in result && ('left' in result || 'right' in result)) {{
        result = treeToArray(result);
    }}

    console.log(JSON.stringify({{ success: true, result }}));
}} catch (e) {{
    console.log(JSON.stringify({{ success: false, error: e.message }}));
}}
'''

    with tempfile.NamedTemporaryFile(mode='w', suffix='.js', delete=False) as f:
        f.write(test_code)
        temp_file = f.name

    try:
        result = subprocess.run(
            ['bun', 'run', temp_file],
            capture_output=True,
            text=True,
            timeout=5
        )
        os.unlink(temp_file)

        if result.returncode == 0 and result.stdout.strip():
            data = json.loads(result.stdout.strip())
            if data.get('success'):
                return data.get('result')
        return None
    except Exception:
        if os.path.exists(temp_file):
            os.unlink(temp_file)
        return None


def add_edge_cases_to_problem(problem_id):
    """Add edge cases to a specific problem's test file."""
    # Get problem info
    problem_files = list(PROBLEMS_DIR.glob(f"{problem_id}_*.json"))
    if not problem_files:
        return 0, f"No problem file for {problem_id}"

    with open(problem_files[0]) as f:
        problem_info = json.load(f)

    func_name = problem_info.get('function_name', '')

    # Get solution file
    solution_files = list(SOLUTIONS_DIR.glob(f"{problem_id}_*.js"))
    if not solution_files:
        return 0, f"No solution file for {problem_id}"
    solution_file = solution_files[0]

    # Get existing tests
    tc_files = list(TESTCASES_DIR.glob(f"{problem_id}_*.json"))
    if not tc_files:
        return 0, f"No testcase file for {problem_id}"
    tc_file = tc_files[0]

    with open(tc_file) as f:
        tc_data = json.load(f)

    existing_tests = tc_data.get('run_tests', []) + tc_data.get('submit_tests', [])
    existing_inputs = {json.dumps(t.get('input'), sort_keys=True) for t in existing_tests}

    # Generate edge cases
    edge_cases = generate_edge_cases_for_problem(problem_id, problem_info, existing_tests)

    added = 0
    for edge_input in edge_cases:
        # Skip if already exists
        input_key = json.dumps(edge_input, sort_keys=True)
        if input_key in existing_inputs:
            continue

        # Compute expected value
        expected = compute_expected_value(solution_file, func_name, edge_input, problem_info)

        if expected is not None:
            new_test = {'input': edge_input, 'expected': expected}
            tc_data.setdefault('submit_tests', []).append(new_test)
            existing_inputs.add(input_key)
            added += 1

    if added > 0:
        with open(tc_file, 'w') as f:
            json.dump(tc_data, f, indent=2)

    return added, None


def main():
    """Generate edge cases for all problems."""
    solution_files = sorted(SOLUTIONS_DIR.glob("*.js"))

    total_added = 0
    problems_updated = 0

    for sol_file in solution_files:
        problem_id = sol_file.name.split('_')[0]

        print(f"Processing {problem_id}...", end=' ', flush=True)

        added, error = add_edge_cases_to_problem(problem_id)

        if error:
            print(f"SKIP ({error})")
        elif added > 0:
            print(f"Added {added} edge cases")
            total_added += added
            problems_updated += 1
        else:
            print("No new cases needed")

    print("\n" + "="*60)
    print("SUMMARY")
    print("="*60)
    print(f"Problems updated: {problems_updated}")
    print(f"Total edge cases added: {total_added}")


if __name__ == '__main__':
    main()
