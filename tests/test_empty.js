
var twoSum = function(nums, target) {
    // Empty - should fail
};

const testCases = [
    { input: [[2, 7, 11, 15], 9], expected: [0, 1] },
];

function runTests() {
    let passed = 0;
    let failed = 0;

    testCases.forEach((tc, i) => {
        try {
            const result = twoSum(...tc.input);
            const resultStr = JSON.stringify(result);
            const expectedStr = JSON.stringify(tc.expected);

            if (resultStr === expectedStr) {
                console.log(`PASSED`);
                passed++;
            } else {
                console.log(`FAILED`);
                failed++;
            }
        } catch (e) {
            console.log(`ERROR: ${e.message}`);
            failed++;
        }
    });

    console.log(`Results: ${passed} passed, ${failed} failed`);
}

runTests();
