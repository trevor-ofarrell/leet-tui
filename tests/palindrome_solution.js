
var isPalindrome = function(x) {
    if (x < 0) return false;
    const str = x.toString();
    return str === str.split('').reverse().join('');
};

const testCases = [
    { input: [121], expected: true },
    { input: [-121], expected: false },
    { input: [10], expected: false },
];

function runTests() {
    let passed = 0;
    let failed = 0;

    testCases.forEach((tc, i) => {
        try {
            const result = isPalindrome(...tc.input);
            const resultStr = JSON.stringify(result);
            const expectedStr = JSON.stringify(tc.expected);

            if (resultStr === expectedStr) {
                console.log(`Test ${i + 1}: PASSED`);
                passed++;
            } else {
                console.log(`Test ${i + 1}: FAILED - Expected ${expectedStr}, got ${resultStr}`);
                failed++;
            }
        } catch (e) {
            console.log(`Test ${i + 1}: ERROR - ${e.message}`);
            failed++;
        }
    });

    console.log(`Results: ${passed} passed, ${failed} failed`);
    process.exit(failed > 0 ? 1 : 0);
}

runTests();
