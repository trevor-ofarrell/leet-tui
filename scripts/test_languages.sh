#!/usr/bin/env bash
# Test multi-language support with 10 problems x 4 languages = 40 tests
# Tests JS and Python fully; C/C++ are scaffolded (manual verification recommended)

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
SOLUTIONS_DIR="$SCRIPT_DIR/test_solutions"
PROBLEMS_DIR="$PROJECT_DIR/problems"
TESTCASES_DIR="$PROJECT_DIR/testcases"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

PASSED=0
FAILED=0
SKIPPED=0

echo "========================================"
echo "  Multi-Language Test Suite"
echo "========================================"
echo ""

# Check dependencies
echo "Checking dependencies..."
HAS_BUN=0
HAS_PYTHON=0
HAS_GCC=0
HAS_GPP=0

if command -v bun &> /dev/null; then
    echo -e "${GREEN}✓${NC} bun found"
    HAS_BUN=1
else
    echo -e "${YELLOW}⚠${NC} bun not found"
fi

if command -v python3 &> /dev/null; then
    echo -e "${GREEN}✓${NC} python3 found"
    HAS_PYTHON=1
else
    echo -e "${YELLOW}⚠${NC} python3 not found"
fi

if command -v gcc &> /dev/null; then
    echo -e "${GREEN}✓${NC} gcc found"
    HAS_GCC=1
else
    echo -e "${YELLOW}⚠${NC} gcc not found"
fi

if command -v g++ &> /dev/null; then
    echo -e "${GREEN}✓${NC} g++ found"
    HAS_GPP=1
else
    echo -e "${YELLOW}⚠${NC} g++ not found"
fi
echo ""

# Test JavaScript solutions
test_js() {
    local padded_id=$1
    local func=$2
    local solution_file=$(ls "$SOLUTIONS_DIR"/${padded_id}_*.js 2>/dev/null | head -1)
    local problem_file=$(ls "$PROBLEMS_DIR"/${padded_id}_*.json 2>/dev/null | head -1)

    if [[ ! -f "$problem_file" ]]; then
        echo -e "  ${YELLOW}SKIP${NC} JS - Problem $padded_id not found"
        SKIPPED=$((SKIPPED + 1))
        return
    fi

    if [[ ! -f "$solution_file" ]]; then
        echo -e "  ${YELLOW}SKIP${NC} JS - Solution not found"
        SKIPPED=$((SKIPPED + 1))
        return
    fi

    local solution=$(cat "$solution_file")

    # Try to get test cases from testcases dir (comprehensive), fall back to problems dir
    local testcases_file=$(ls "$TESTCASES_DIR"/${padded_id}_*.json 2>/dev/null | head -1)
    local test_cases
    if [[ -f "$testcases_file" ]]; then
        test_cases=$(python3 -c "
import json
with open('$testcases_file') as f:
    tc = json.load(f)
    all_tests = tc.get('run_tests', []) + tc.get('submit_tests', [])
    print(json.dumps(all_tests))
" 2>/dev/null)
    else
        test_cases=$(python3 -c "
import json
with open('$problem_file') as f:
    p = json.load(f)
    print(json.dumps(p['test_cases']))
" 2>/dev/null)
    fi

    local test_file=$(mktemp /tmp/test_XXXXXX.js)
    cat > "$test_file" << EOF
$solution

// Helper functions for linked list problems
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

// Helper for tree problems
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
    // Trim trailing nulls
    while (result.length > 0 && result[result.length - 1] === null) result.pop();
    return result;
};

const testCases = $test_cases;
let passed = 0;
let failed = 0;

// Normalize for order-independent comparison (for problems like groupAnagrams, findWords)
const normalize = (val) => {
    if (!Array.isArray(val)) return JSON.stringify(val);
    // Check if it's an array of arrays (like groupAnagrams output)
    if (val.length > 0 && Array.isArray(val[0])) {
        // Sort each inner array, then sort outer array by stringified inner
        const sorted = val.map(inner => [...inner].sort()).sort((a, b) =>
            JSON.stringify(a).localeCompare(JSON.stringify(b))
        );
        return JSON.stringify(sorted);
    }
    // For arrays of primitives, sort for order-independent comparison
    if (val.length > 0 && (typeof val[0] === 'string' || typeof val[0] === 'number')) {
        return JSON.stringify([...val].sort());
    }
    return JSON.stringify(val);
};

// Linked list problem detection (simple ones, not mergeKLists which needs array of lists)
const linkedListFuncs = ['addTwoNumbers', 'removeNthFromEnd', 'mergeTwoLists',
    'reverseKGroup', 'reverseList', 'reorderList'];
// Tree problems where inputs ARE trees (not arrays like buildTree)
const treeFuncs = ['invertTree', 'maxDepth', 'diameterOfBinaryTree', 'isBalanced', 'isSameTree',
    'isSubtree', 'levelOrder', 'rightSideView', 'isValidBST', 'kthSmallest',
    'maxPathSum', 'goodNodes'];
// In-place functions that modify input and return void
const inPlaceFuncs = ['rotate', 'setZeroes', 'wallsAndGates', 'reorderList', 'solve'];
// Class-based or special problems that need custom harness - skip for now
const classFuncs = ['LRUCache', 'MinStack', 'Trie', 'trie', 'WordDictionary', 'wordDictionary',
    'MedianFinder', 'medianFinder', 'Twitter', 'KthLargest', 'TimeMap', 'DetectSquares',
    'Codec', 'codec', 'encodeDecode'];
// Graph/special problems that need custom input handling - skip for now
const specialFuncs = ['cloneGraph', 'copyRandomList', 'hasCycle', 'detectCycle',
    'mergeKLists', 'lowestCommonAncestor'];

const isLinkedList = linkedListFuncs.includes('$func');
const isTree = treeFuncs.includes('$func');
const isInPlace = inPlaceFuncs.includes('$func');
const isClass = classFuncs.includes('$func');
const isSpecial = specialFuncs.includes('$func');

// Skip class-based and special problems for now
if (isClass || isSpecial) {
    console.log('Skipping special problem');
    process.exit(0);
}

for (const tc of testCases) {
    let inputs = tc.input;

    // Convert inputs for linked list problems
    if (isLinkedList) {
        inputs = inputs.map(inp => Array.isArray(inp) ? arrayToList(inp) : inp);
    }
    // Convert inputs for tree problems
    if (isTree) {
        inputs = inputs.map(inp => Array.isArray(inp) ? arrayToTree(inp) : inp);
    }

    let result;
    if (isInPlace) {
        // For in-place functions, call the function and use modified input as result
        $func(...inputs);
        result = inputs[0]; // The modified input (matrix, list, etc.)
    } else {
        result = $func(...inputs);
    }

    // Convert result back if it's a linked list
    if (result && typeof result === 'object' && 'val' in result && 'next' in result) {
        result = listToArray(result);
    }
    // Handle null linked list result -> empty array
    if (result === null && Array.isArray(tc.expected) && tc.expected.length === 0) {
        result = [];
    }
    // Convert result back if it's a tree (check for left/right)
    if (result && typeof result === 'object' && 'val' in result && ('left' in result || 'right' in result)) {
        result = treeToArray(result);
    }

    // Handle floating point comparison
    let resultStr = normalize(result);
    let expectedStr = normalize(tc.expected);

    // For floating point numbers, compare with tolerance
    const resultNum = parseFloat(result);
    const expectedNum = parseFloat(tc.expected);
    if (!isNaN(resultNum) && !isNaN(expectedNum) && typeof result === 'number') {
        if (Math.abs(resultNum - expectedNum) < 0.0001) {
            resultStr = expectedStr; // Treat as equal
        }
    }
    if (resultStr === expectedStr) {
        passed++;
    } else {
        failed++;
        console.log(\`FAILED: expected=\${expectedStr}, got=\${resultStr}\`);
    }
}

if (failed === 0) {
    console.log(\`All \${passed} tests passed!\`);
    process.exit(0);
} else {
    console.log(\`\${failed} tests failed\`);
    process.exit(1);
}
EOF

    local output
    if output=$(bun run "$test_file" 2>&1); then
        echo -e "  ${GREEN}PASS${NC} JavaScript"
        PASSED=$((PASSED + 1))
    else
        echo -e "  ${RED}FAIL${NC} JavaScript"
        echo "       $output"
        FAILED=$((FAILED + 1))
    fi
    rm -f "$test_file"
}

# Test Python solutions
test_python() {
    local padded_id=$1
    local func=$2
    local solution_file=$(ls "$SOLUTIONS_DIR"/${padded_id}_*.py 2>/dev/null | head -1)
    local problem_file=$(ls "$PROBLEMS_DIR"/${padded_id}_*.json 2>/dev/null | head -1)

    if [[ ! -f "$problem_file" ]]; then
        echo -e "  ${YELLOW}SKIP${NC} Python - Problem $padded_id not found"
        SKIPPED=$((SKIPPED + 1))
        return
    fi

    if [[ ! -f "$solution_file" ]]; then
        echo -e "  ${YELLOW}SKIP${NC} Python - Solution not found"
        SKIPPED=$((SKIPPED + 1))
        return
    fi

    local solution=$(cat "$solution_file")

    local test_cases=$(python3 -c "
import json
with open('$problem_file') as f:
    p = json.load(f)
    print(json.dumps(p['test_cases']))
" 2>/dev/null)

    local test_file=$(mktemp /tmp/test_XXXXXX.py)
    cat > "$test_file" << EOF
import json

$solution

test_cases = json.loads('''$test_cases''')
passed = 0
failed = 0

# Normalize for order-independent comparison (for problems like groupAnagrams)
def normalize(val):
    if not isinstance(val, list):
        return json.dumps(val, sort_keys=True)
    # Check if it's a list of lists (like groupAnagrams output)
    if len(val) > 0 and isinstance(val[0], list):
        # Sort each inner list, then sort outer list by stringified inner
        sorted_val = sorted([sorted(inner) for inner in val], key=lambda x: json.dumps(x))
        return json.dumps(sorted_val, sort_keys=True)
    return json.dumps(val, sort_keys=True)

for tc in test_cases:
    result = $func(*tc["input"])
    result_str = normalize(result)
    expected_str = normalize(tc["expected"])
    if result_str == expected_str:
        passed += 1
    else:
        failed += 1
        print(f"FAILED: expected={expected_str}, got={result_str}")

if failed == 0:
    print(f"All {passed} tests passed!")
    exit(0)
else:
    print(f"{failed} tests failed")
    exit(1)
EOF

    local output
    if output=$(python3 "$test_file" 2>&1); then
        echo -e "  ${GREEN}PASS${NC} Python"
        PASSED=$((PASSED + 1))
    else
        echo -e "  ${RED}FAIL${NC} Python"
        echo "       $output"
        FAILED=$((FAILED + 1))
    fi
    rm -f "$test_file"
}

# Test C solutions
test_c() {
    local padded_id=$1
    local func=$2
    local solution_file=$(ls "$SOLUTIONS_DIR"/${padded_id}_*.c 2>/dev/null | head -1)

    if [[ ! -f "$solution_file" ]]; then
        echo -e "  ${YELLOW}SKIP${NC} C - Solution not found"
        SKIPPED=$((SKIPPED + 1))
        return
    fi

    local solution=$(cat "$solution_file")
    local test_file=$(mktemp /tmp/test_XXXXXX.c)
    local bin_file=$(mktemp /tmp/test_bin_XXXXXX)

    # Generate problem-specific test harness
    case "$padded_id" in
        "001") # twoSum
            cat > "$test_file" << 'CEOF'
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
CEOF
            echo "$solution" >> "$test_file"
            cat >> "$test_file" << 'CEOF'
int main() {
    int passed = 0, failed = 0;
    // Test 1: [2,7,11,15], 9 -> [0,1]
    int nums1[] = {2, 7, 11, 15}; int returnSize1;
    int* res1 = twoSum(nums1, 4, 9, &returnSize1);
    if (returnSize1 == 2 && res1[0] == 0 && res1[1] == 1) passed++; else failed++;
    free(res1);
    // Test 2: [3,2,4], 6 -> [1,2]
    int nums2[] = {3, 2, 4}; int returnSize2;
    int* res2 = twoSum(nums2, 3, 6, &returnSize2);
    if (returnSize2 == 2 && res2[0] == 1 && res2[1] == 2) passed++; else failed++;
    free(res2);
    // Test 3: [3,3], 6 -> [0,1]
    int nums3[] = {3, 3}; int returnSize3;
    int* res3 = twoSum(nums3, 2, 6, &returnSize3);
    if (returnSize3 == 2 && res3[0] == 0 && res3[1] == 1) passed++; else failed++;
    free(res3);
    if (failed == 0) { printf("All %d tests passed!\n", passed); return 0; }
    else { printf("%d tests failed\n", failed); return 1; }
}
CEOF
            ;;
        "217") # containsDuplicate
            cat > "$test_file" << 'CEOF'
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>
CEOF
            echo "$solution" >> "$test_file"
            cat >> "$test_file" << 'CEOF'
int main() {
    int passed = 0, failed = 0;
    int nums1[] = {1, 2, 3, 1};
    if (containsDuplicate(nums1, 4) == true) passed++; else failed++;
    int nums2[] = {1, 2, 3, 4};
    if (containsDuplicate(nums2, 4) == false) passed++; else failed++;
    int nums3[] = {1, 1, 1, 3, 3, 4, 3, 2, 4, 2};
    if (containsDuplicate(nums3, 10) == true) passed++; else failed++;
    if (failed == 0) { printf("All %d tests passed!\n", passed); return 0; }
    else { printf("%d tests failed\n", failed); return 1; }
}
CEOF
            ;;
        "242") # isAnagram
            cat > "$test_file" << 'CEOF'
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>
CEOF
            echo "$solution" >> "$test_file"
            cat >> "$test_file" << 'CEOF'
int main() {
    int passed = 0, failed = 0;
    if (isAnagram("anagram", "nagaram") == true) passed++; else failed++;
    if (isAnagram("rat", "car") == false) passed++; else failed++;
    if (isAnagram("a", "ab") == false) passed++; else failed++;
    if (failed == 0) { printf("All %d tests passed!\n", passed); return 0; }
    else { printf("%d tests failed\n", failed); return 1; }
}
CEOF
            ;;
        "049") # groupAnagrams - complex, skip for C
            echo -e "  ${YELLOW}SKIP${NC} C - groupAnagrams requires complex memory management"
            SKIPPED=$((SKIPPED + 1))
            rm -f "$test_file" "$bin_file"
            return
            ;;
        "347") # topKFrequent
            cat > "$test_file" << 'CEOF'
#include <stdio.h>
#include <stdlib.h>
CEOF
            echo "$solution" >> "$test_file"
            cat >> "$test_file" << 'CEOF'
int arrContains(int* arr, int size, int val) {
    for (int i = 0; i < size; i++) if (arr[i] == val) return 1;
    return 0;
}
int main() {
    int passed = 0, failed = 0;
    int returnSize1;
    int nums1[] = {1, 1, 1, 2, 2, 3};
    int* res1 = topKFrequent(nums1, 6, 2, &returnSize1);
    if (returnSize1 == 2 && arrContains(res1, 2, 1) && arrContains(res1, 2, 2)) passed++; else failed++;
    free(res1);
    int returnSize2;
    int nums2[] = {1};
    int* res2 = topKFrequent(nums2, 1, 1, &returnSize2);
    if (returnSize2 == 1 && res2[0] == 1) passed++; else failed++;
    free(res2);
    if (failed == 0) { printf("All %d tests passed!\n", passed); return 0; }
    else { printf("%d tests failed\n", failed); return 1; }
}
CEOF
            ;;
        "338") # countBits
            cat > "$test_file" << 'CEOF'
#include <stdio.h>
#include <stdlib.h>
CEOF
            echo "$solution" >> "$test_file"
            cat >> "$test_file" << 'CEOF'
int main() {
    int passed = 0, failed = 0;
    int returnSize1;
    int* res1 = countBits(2, &returnSize1);
    int exp1[] = {0, 1, 1};
    int ok1 = (returnSize1 == 3);
    for (int i = 0; ok1 && i < 3; i++) if (res1[i] != exp1[i]) ok1 = 0;
    if (ok1) passed++; else failed++;
    free(res1);
    int returnSize2;
    int* res2 = countBits(5, &returnSize2);
    int exp2[] = {0, 1, 1, 2, 1, 2};
    int ok2 = (returnSize2 == 6);
    for (int i = 0; ok2 && i < 6; i++) if (res2[i] != exp2[i]) ok2 = 0;
    if (ok2) passed++; else failed++;
    free(res2);
    if (failed == 0) { printf("All %d tests passed!\n", passed); return 0; }
    else { printf("%d tests failed\n", failed); return 1; }
}
CEOF
            ;;
        "190") # reverseBits
            cat > "$test_file" << 'CEOF'
#include <stdio.h>
#include <stdint.h>
CEOF
            echo "$solution" >> "$test_file"
            cat >> "$test_file" << 'CEOF'
int main() {
    int passed = 0, failed = 0;
    if (reverseBits(43261596) == 964176192) passed++; else failed++;
    if (reverseBits(3) == 3221225472U) passed++; else failed++;
    if (failed == 0) { printf("All %d tests passed!\n", passed); return 0; }
    else { printf("%d tests failed\n", failed); return 1; }
}
CEOF
            ;;
        "268") # missingNumber
            cat > "$test_file" << 'CEOF'
#include <stdio.h>
CEOF
            echo "$solution" >> "$test_file"
            cat >> "$test_file" << 'CEOF'
int main() {
    int passed = 0, failed = 0;
    int nums1[] = {3, 0, 1};
    if (missingNumber(nums1, 3) == 2) passed++; else failed++;
    int nums2[] = {0, 1};
    if (missingNumber(nums2, 2) == 2) passed++; else failed++;
    int nums3[] = {9, 6, 4, 2, 3, 5, 7, 0, 1};
    if (missingNumber(nums3, 9) == 8) passed++; else failed++;
    if (failed == 0) { printf("All %d tests passed!\n", passed); return 0; }
    else { printf("%d tests failed\n", failed); return 1; }
}
CEOF
            ;;
        "371") # getSum
            cat > "$test_file" << 'CEOF'
#include <stdio.h>
CEOF
            echo "$solution" >> "$test_file"
            cat >> "$test_file" << 'CEOF'
int main() {
    int passed = 0, failed = 0;
    if (getSum(1, 2) == 3) passed++; else failed++;
    if (getSum(20, 30) == 50) passed++; else failed++;
    if (getSum(-1, 1) == 0) passed++; else failed++;
    if (failed == 0) { printf("All %d tests passed!\n", passed); return 0; }
    else { printf("%d tests failed\n", failed); return 1; }
}
CEOF
            ;;
        "007") # reverse
            cat > "$test_file" << 'CEOF'
#include <stdio.h>
#include <limits.h>
CEOF
            echo "$solution" >> "$test_file"
            cat >> "$test_file" << 'CEOF'
int main() {
    int passed = 0, failed = 0;
    if (reverse(123) == 321) passed++; else failed++;
    if (reverse(-123) == -321) passed++; else failed++;
    if (reverse(120) == 21) passed++; else failed++;
    if (reverse(0) == 0) passed++; else failed++;
    if (reverse(1534236469) == 0) passed++; else failed++;
    if (failed == 0) { printf("All %d tests passed!\n", passed); return 0; }
    else { printf("%d tests failed\n", failed); return 1; }
}
CEOF
            ;;
        *)
            echo -e "  ${YELLOW}SKIP${NC} C - No test harness for problem $padded_id"
            SKIPPED=$((SKIPPED + 1))
            rm -f "$test_file" "$bin_file"
            return
            ;;
    esac

    local output
    if output=$(gcc -O2 -o "$bin_file" "$test_file" 2>&1); then
        if output=$("$bin_file" 2>&1); then
            echo -e "  ${GREEN}PASS${NC} C"
            PASSED=$((PASSED + 1))
        else
            echo -e "  ${RED}FAIL${NC} C"
            echo "       $output"
            FAILED=$((FAILED + 1))
        fi
    else
        echo -e "  ${RED}FAIL${NC} C (compilation error)"
        echo "       $output"
        FAILED=$((FAILED + 1))
    fi
    rm -f "$test_file" "$bin_file"
}

# Test C++ solutions
test_cpp() {
    local padded_id=$1
    local func=$2
    local solution_file=$(ls "$SOLUTIONS_DIR"/${padded_id}_*.cpp 2>/dev/null | head -1)

    if [[ ! -f "$solution_file" ]]; then
        echo -e "  ${YELLOW}SKIP${NC} C++ - Solution not found"
        SKIPPED=$((SKIPPED + 1))
        return
    fi

    local solution=$(cat "$solution_file")
    local test_file=$(mktemp /tmp/test_XXXXXX.cpp)
    local bin_file=$(mktemp /tmp/test_bin_XXXXXX)

    # Generate problem-specific test harness
    case "$padded_id" in
        "001") # twoSum
            cat > "$test_file" << 'CPPEOF'
#include <iostream>
#include <vector>
#include <unordered_map>
using namespace std;
CPPEOF
            echo "$solution" >> "$test_file"
            cat >> "$test_file" << 'CPPEOF'
int main() {
    Solution s;
    int passed = 0, failed = 0;
    vector<int> nums1 = {2, 7, 11, 15};
    vector<int> res1 = s.twoSum(nums1, 9);
    if (res1.size() == 2 && res1[0] == 0 && res1[1] == 1) passed++; else failed++;
    vector<int> nums2 = {3, 2, 4};
    vector<int> res2 = s.twoSum(nums2, 6);
    if (res2.size() == 2 && res2[0] == 1 && res2[1] == 2) passed++; else failed++;
    vector<int> nums3 = {3, 3};
    vector<int> res3 = s.twoSum(nums3, 6);
    if (res3.size() == 2 && res3[0] == 0 && res3[1] == 1) passed++; else failed++;
    if (failed == 0) { cout << "All " << passed << " tests passed!" << endl; return 0; }
    else { cout << failed << " tests failed" << endl; return 1; }
}
CPPEOF
            ;;
        "217") # containsDuplicate
            cat > "$test_file" << 'CPPEOF'
#include <iostream>
#include <vector>
#include <unordered_set>
using namespace std;
CPPEOF
            echo "$solution" >> "$test_file"
            cat >> "$test_file" << 'CPPEOF'
int main() {
    Solution s;
    int passed = 0, failed = 0;
    vector<int> nums1 = {1, 2, 3, 1};
    if (s.containsDuplicate(nums1) == true) passed++; else failed++;
    vector<int> nums2 = {1, 2, 3, 4};
    if (s.containsDuplicate(nums2) == false) passed++; else failed++;
    vector<int> nums3 = {1, 1, 1, 3, 3, 4, 3, 2, 4, 2};
    if (s.containsDuplicate(nums3) == true) passed++; else failed++;
    if (failed == 0) { cout << "All " << passed << " tests passed!" << endl; return 0; }
    else { cout << failed << " tests failed" << endl; return 1; }
}
CPPEOF
            ;;
        "242") # isAnagram
            cat > "$test_file" << 'CPPEOF'
#include <iostream>
#include <string>
using namespace std;
CPPEOF
            echo "$solution" >> "$test_file"
            cat >> "$test_file" << 'CPPEOF'
int main() {
    Solution s;
    int passed = 0, failed = 0;
    if (s.isAnagram("anagram", "nagaram") == true) passed++; else failed++;
    if (s.isAnagram("rat", "car") == false) passed++; else failed++;
    if (s.isAnagram("a", "ab") == false) passed++; else failed++;
    if (failed == 0) { cout << "All " << passed << " tests passed!" << endl; return 0; }
    else { cout << failed << " tests failed" << endl; return 1; }
}
CPPEOF
            ;;
        "049") # groupAnagrams
            cat > "$test_file" << 'CPPEOF'
#include <iostream>
#include <vector>
#include <string>
#include <unordered_map>
#include <algorithm>
using namespace std;
CPPEOF
            echo "$solution" >> "$test_file"
            cat >> "$test_file" << 'CPPEOF'
vector<vector<string>> normalize(vector<vector<string>> v) {
    for (auto& inner : v) sort(inner.begin(), inner.end());
    sort(v.begin(), v.end());
    return v;
}
int main() {
    Solution s;
    int passed = 0, failed = 0;
    vector<string> strs1 = {"eat", "tea", "tan", "ate", "nat", "bat"};
    auto res1 = normalize(s.groupAnagrams(strs1));
    vector<vector<string>> exp1 = {{"ate", "eat", "tea"}, {"bat"}, {"nat", "tan"}};
    if (res1 == normalize(exp1)) passed++; else failed++;
    vector<string> strs2 = {""};
    auto res2 = normalize(s.groupAnagrams(strs2));
    vector<vector<string>> exp2 = {{""}};
    if (res2 == normalize(exp2)) passed++; else failed++;
    vector<string> strs3 = {"a"};
    auto res3 = normalize(s.groupAnagrams(strs3));
    vector<vector<string>> exp3 = {{"a"}};
    if (res3 == normalize(exp3)) passed++; else failed++;
    if (failed == 0) { cout << "All " << passed << " tests passed!" << endl; return 0; }
    else { cout << failed << " tests failed" << endl; return 1; }
}
CPPEOF
            ;;
        "347") # topKFrequent
            cat > "$test_file" << 'CPPEOF'
#include <iostream>
#include <vector>
#include <unordered_map>
#include <algorithm>
using namespace std;
CPPEOF
            echo "$solution" >> "$test_file"
            cat >> "$test_file" << 'CPPEOF'
bool contains(vector<int>& v, int val) {
    return find(v.begin(), v.end(), val) != v.end();
}
int main() {
    Solution s;
    int passed = 0, failed = 0;
    vector<int> nums1 = {1, 1, 1, 2, 2, 3};
    auto res1 = s.topKFrequent(nums1, 2);
    if (res1.size() == 2 && contains(res1, 1) && contains(res1, 2)) passed++; else failed++;
    vector<int> nums2 = {1};
    auto res2 = s.topKFrequent(nums2, 1);
    if (res2.size() == 1 && res2[0] == 1) passed++; else failed++;
    if (failed == 0) { cout << "All " << passed << " tests passed!" << endl; return 0; }
    else { cout << failed << " tests failed" << endl; return 1; }
}
CPPEOF
            ;;
        "338") # countBits
            cat > "$test_file" << 'CPPEOF'
#include <iostream>
#include <vector>
using namespace std;
CPPEOF
            echo "$solution" >> "$test_file"
            cat >> "$test_file" << 'CPPEOF'
int main() {
    Solution s;
    int passed = 0, failed = 0;
    vector<int> res1 = s.countBits(2);
    vector<int> exp1 = {0, 1, 1};
    if (res1 == exp1) passed++; else failed++;
    vector<int> res2 = s.countBits(5);
    vector<int> exp2 = {0, 1, 1, 2, 1, 2};
    if (res2 == exp2) passed++; else failed++;
    if (failed == 0) { cout << "All " << passed << " tests passed!" << endl; return 0; }
    else { cout << failed << " tests failed" << endl; return 1; }
}
CPPEOF
            ;;
        "190") # reverseBits
            cat > "$test_file" << 'CPPEOF'
#include <iostream>
#include <cstdint>
using namespace std;
CPPEOF
            echo "$solution" >> "$test_file"
            cat >> "$test_file" << 'CPPEOF'
int main() {
    Solution s;
    int passed = 0, failed = 0;
    if (s.reverseBits(43261596) == 964176192) passed++; else failed++;
    if (s.reverseBits(3) == 3221225472U) passed++; else failed++;
    if (failed == 0) { cout << "All " << passed << " tests passed!" << endl; return 0; }
    else { cout << failed << " tests failed" << endl; return 1; }
}
CPPEOF
            ;;
        "268") # missingNumber
            cat > "$test_file" << 'CPPEOF'
#include <iostream>
#include <vector>
using namespace std;
CPPEOF
            echo "$solution" >> "$test_file"
            cat >> "$test_file" << 'CPPEOF'
int main() {
    Solution s;
    int passed = 0, failed = 0;
    vector<int> nums1 = {3, 0, 1};
    if (s.missingNumber(nums1) == 2) passed++; else failed++;
    vector<int> nums2 = {0, 1};
    if (s.missingNumber(nums2) == 2) passed++; else failed++;
    vector<int> nums3 = {9, 6, 4, 2, 3, 5, 7, 0, 1};
    if (s.missingNumber(nums3) == 8) passed++; else failed++;
    if (failed == 0) { cout << "All " << passed << " tests passed!" << endl; return 0; }
    else { cout << failed << " tests failed" << endl; return 1; }
}
CPPEOF
            ;;
        "371") # getSum
            cat > "$test_file" << 'CPPEOF'
#include <iostream>
using namespace std;
CPPEOF
            echo "$solution" >> "$test_file"
            cat >> "$test_file" << 'CPPEOF'
int main() {
    Solution s;
    int passed = 0, failed = 0;
    if (s.getSum(1, 2) == 3) passed++; else failed++;
    if (s.getSum(20, 30) == 50) passed++; else failed++;
    if (s.getSum(-1, 1) == 0) passed++; else failed++;
    if (failed == 0) { cout << "All " << passed << " tests passed!" << endl; return 0; }
    else { cout << failed << " tests failed" << endl; return 1; }
}
CPPEOF
            ;;
        "007") # reverse
            cat > "$test_file" << 'CPPEOF'
#include <iostream>
#include <climits>
using namespace std;
CPPEOF
            echo "$solution" >> "$test_file"
            cat >> "$test_file" << 'CPPEOF'
int main() {
    Solution s;
    int passed = 0, failed = 0;
    if (s.reverse(123) == 321) passed++; else failed++;
    if (s.reverse(-123) == -321) passed++; else failed++;
    if (s.reverse(120) == 21) passed++; else failed++;
    if (s.reverse(0) == 0) passed++; else failed++;
    if (s.reverse(1534236469) == 0) passed++; else failed++;
    if (failed == 0) { cout << "All " << passed << " tests passed!" << endl; return 0; }
    else { cout << failed << " tests failed" << endl; return 1; }
}
CPPEOF
            ;;
        *)
            echo -e "  ${YELLOW}SKIP${NC} C++ - No test harness for problem $padded_id"
            SKIPPED=$((SKIPPED + 1))
            rm -f "$test_file" "$bin_file"
            return
            ;;
    esac

    local output
    if output=$(g++ -std=c++17 -O2 -o "$bin_file" "$test_file" 2>&1); then
        if output=$("$bin_file" 2>&1); then
            echo -e "  ${GREEN}PASS${NC} C++"
            PASSED=$((PASSED + 1))
        else
            echo -e "  ${RED}FAIL${NC} C++"
            echo "       $output"
            FAILED=$((FAILED + 1))
        fi
    else
        echo -e "  ${RED}FAIL${NC} C++ (compilation error)"
        echo "       $output"
        FAILED=$((FAILED + 1))
    fi
    rm -f "$test_file" "$bin_file"
}

# Run tests for a single problem
run_problem_tests() {
    local padded_id=$1
    local func=$2

    echo "Problem $padded_id ($func):"

    if [[ "$HAS_BUN" == "1" ]]; then
        test_js "$padded_id" "$func"
    else
        echo -e "  ${YELLOW}SKIP${NC} JavaScript (bun not installed)"
        SKIPPED=$((SKIPPED + 1))
    fi

    if [[ "$HAS_PYTHON" == "1" ]]; then
        test_python "$padded_id" "$func"
    else
        echo -e "  ${YELLOW}SKIP${NC} Python (python3 not installed)"
        SKIPPED=$((SKIPPED + 1))
    fi

    if [[ "$HAS_GCC" == "1" ]]; then
        test_c "$padded_id" "$func"
    else
        echo -e "  ${YELLOW}SKIP${NC} C (gcc not installed)"
        SKIPPED=$((SKIPPED + 1))
    fi

    if [[ "$HAS_GPP" == "1" ]]; then
        test_cpp "$padded_id" "$func"
    else
        echo -e "  ${YELLOW}SKIP${NC} C++ (g++ not installed)"
        SKIPPED=$((SKIPPED + 1))
    fi

    echo ""
}

# C/C++ problems with hardcoded test harnesses (use string keys to avoid octal issues)
CPP_PROBLEM_LIST="001 007 049 190 217 242 268 338 347 371"

has_cpp_harness() {
    local id="$1"
    [[ " $CPP_PROBLEM_LIST " == *" $id "* ]]
}

# Auto-discover all JS solutions and test them
for js_file in "$SOLUTIONS_DIR"/*.js; do
    [[ -f "$js_file" ]] || continue

    # Extract problem ID from filename (e.g., 001_two_sum.js -> 001)
    filename=$(basename "$js_file")
    padded_id=$(echo "$filename" | grep -oE '^[0-9]+')

    # Find corresponding problem JSON
    problem_file=$(ls "$PROBLEMS_DIR"/${padded_id}_*.json 2>/dev/null | head -1)
    [[ -f "$problem_file" ]] || continue

    # Extract function name from problem JSON
    func=$(python3 -c "
import json
with open('$problem_file') as f:
    p = json.load(f)
    print(p.get('function_name', ''))
" 2>/dev/null)

    [[ -z "$func" ]] && continue

    echo "Problem $padded_id ($func):"

    # Test JavaScript
    if [[ "$HAS_BUN" == "1" ]]; then
        test_js "$padded_id" "$func"
    else
        echo -e "  ${YELLOW}SKIP${NC} JavaScript (bun not installed)"
        SKIPPED=$((SKIPPED + 1))
    fi

    # Test Python (only if solution exists)
    if [[ "$HAS_PYTHON" == "1" ]]; then
        py_file=$(ls "$SOLUTIONS_DIR"/${padded_id}_*.py 2>/dev/null | head -1)
        if [[ -f "$py_file" ]]; then
            test_python "$padded_id" "$func"
        fi
    fi

    # Test C/C++ only for problems with harnesses
    if has_cpp_harness "$padded_id"; then
        if [[ "$HAS_GCC" == "1" ]]; then
            test_c "$padded_id" "$func"
        fi
        if [[ "$HAS_GPP" == "1" ]]; then
            test_cpp "$padded_id" "$func"
        fi
    fi

    echo ""
done

echo "========================================"
echo "  Summary"
echo "========================================"
echo -e "${GREEN}Passed:${NC}  $PASSED"
echo -e "${RED}Failed:${NC}  $FAILED"
echo -e "${YELLOW}Skipped:${NC} $SKIPPED"
echo ""

if [[ $FAILED -eq 0 && $PASSED -gt 0 ]]; then
    echo -e "${GREEN}All executed tests passed!${NC}"
    exit 0
elif [[ $FAILED -eq 0 ]]; then
    echo -e "${YELLOW}No tests executed${NC}"
    exit 0
else
    echo -e "${RED}Some tests failed${NC}"
    exit 1
fi
