/**
 * Problem 1: Two Sum
 * Difficulty: Easy
 */

var twoSum = function(nums, target) {
    const map = new Map();
    for (let i = 0; i < nums.length; i++) {
        const complement = target - nums[i];
        if (map.has(complement)) {
            return [map.get(complement), i];
        }
        map.set(nums[i], i);
    }
    return [];
};

// ============== TEST RUNNER (do not modify below) ==============
const testCases = [
    { input: [[2, 7, 11, 15], 9], expected: [0, 1] },
    { input: [[3, 2, 4], 6], expected: [1, 2] },
    { input: [[3, 3], 6], expected: [0, 1] },
];

function runTests() {
    console.log('\n' + '='.repeat(50));
    console.log('Running tests for: Two Sum');
    console.log('='.repeat(50) + '\n');

    let passed = 0;
    let failed = 0;

    testCases.forEach((tc, i) => {
        try {
            const result = twoSum(...tc.input);
            const resultStr = JSON.stringify(result);
            const expectedStr = JSON.stringify(tc.expected);

            const isEqual = resultStr === expectedStr;

            if (isEqual) {
                console.log(`✓ Test ${i + 1}: PASSED`);
                passed++;
            } else {
                console.log(`✗ Test ${i + 1}: FAILED`);
                console.log(`  Input:    ${JSON.stringify(tc.input)}`);
                console.log(`  Expected: ${expectedStr}`);
                console.log(`  Got:      ${resultStr}`);
                failed++;
            }
        } catch (e) {
            console.log(`✗ Test ${i + 1}: ERROR`);
            console.log(`  ${e.message}`);
            failed++;
        }
    });

    console.log('\n' + '-'.repeat(50));
    console.log(`Results: ${passed} passed, ${failed} failed out of ${testCases.length} tests`);
    console.log('-'.repeat(50) + '\n');

    if (failed === 0) {
        console.log('🎉 All tests passed!');
    }

    // Exit with error code if any failed
    process.exit(failed > 0 ? 1 : 0);
}

runTests();
