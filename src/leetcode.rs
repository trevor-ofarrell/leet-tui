use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Problem {
    pub id: u32,
    pub title: String,
    pub difficulty: String,
    pub description: String,
    pub examples: Vec<String>,
    pub function_signature: String,
}

pub struct LeetCodeClient;

impl LeetCodeClient {
    pub fn new() -> Self {
        Self
    }

    pub fn get_problems(&self) -> Result<Vec<Problem>> {
        // For now, return sample problems
        // In a real implementation, you would fetch from LeetCode's GraphQL API
        Ok(vec![
            Problem {
                id: 1,
                title: "Two Sum".to_string(),
                difficulty: "Easy".to_string(),
                description: r#"Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

You can return the answer in any order.

Constraints:
- 2 <= nums.length <= 10^4
- -10^9 <= nums[i] <= 10^9
- -10^9 <= target <= 10^9
- Only one valid answer exists.

Follow-up: Can you come up with an algorithm that is less than O(n^2) time complexity?"#.to_string(),
                examples: vec![
                    r#"Input: nums = [2,7,11,15], target = 9
Output: [0,1]
Explanation: Because nums[0] + nums[1] == 9, we return [0, 1]."#.to_string(),
                    r#"Input: nums = [3,2,4], target = 6
Output: [1,2]"#.to_string(),
                    r#"Input: nums = [3,3], target = 6
Output: [0,1]"#.to_string(),
                ],
                function_signature: "var twoSum = function(nums, target)".to_string(),
            },
            Problem {
                id: 9,
                title: "Palindrome Number".to_string(),
                difficulty: "Easy".to_string(),
                description: r#"Given an integer x, return true if x is a palindrome, and false otherwise.

Constraints:
- -2^31 <= x <= 2^31 - 1

Follow up: Could you solve it without converting the integer to a string?"#.to_string(),
                examples: vec![
                    r#"Input: x = 121
Output: true
Explanation: 121 reads as 121 from left to right and from right to left."#.to_string(),
                    r#"Input: x = -121
Output: false
Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome."#.to_string(),
                    r#"Input: x = 10
Output: false
Explanation: Reads 01 from right to left. Therefore it is not a palindrome."#.to_string(),
                ],
                function_signature: "var isPalindrome = function(x)".to_string(),
            },
            Problem {
                id: 13,
                title: "Roman to Integer".to_string(),
                difficulty: "Easy".to_string(),
                description: r#"Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.

Symbol       Value
I             1
V             5
X             10
L             50
C             100
D             500
M             1000

For example, 2 is written as II in Roman numeral, just two ones added together. 12 is written as XII, which is simply X + II. The number 27 is written as XXVII, which is XX + V + II.

Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not IIII. Instead, the number four is written as IV. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:

- I can be placed before V (5) and X (10) to make 4 and 9.
- X can be placed before L (50) and C (100) to make 40 and 90.
- C can be placed before D (500) and M (1000) to make 400 and 900.

Given a roman numeral, convert it to an integer.

Constraints:
- 1 <= s.length <= 15
- s contains only the characters ('I', 'V', 'X', 'L', 'C', 'D', 'M').
- It is guaranteed that s is a valid roman numeral in the range [1, 3999]."#.to_string(),
                examples: vec![
                    r#"Input: s = "III"
Output: 3
Explanation: III = 3."#.to_string(),
                    r#"Input: s = "LVIII"
Output: 58
Explanation: L = 50, V= 5, III = 3."#.to_string(),
                    r#"Input: s = "MCMXCIV"
Output: 1994
Explanation: M = 1000, CM = 900, XC = 90 and IV = 4."#.to_string(),
                ],
                function_signature: "var romanToInt = function(s)".to_string(),
            },
            Problem {
                id: 15,
                title: "3Sum".to_string(),
                difficulty: "Medium".to_string(),
                description: r#"Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]] such that i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.

Notice that the solution set must not contain duplicate triplets.

Constraints:
- 3 <= nums.length <= 3000
- -10^5 <= nums[i] <= 10^5"#.to_string(),
                examples: vec![
                    r#"Input: nums = [-1,0,1,2,-1,-4]
Output: [[-1,-1,2],[-1,0,1]]
Explanation:
nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0.
nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0.
nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0.
The distinct triplets are [-1,0,1] and [-1,-1,2].
Notice that the order of the output and the order of the triplets does not matter."#.to_string(),
                    r#"Input: nums = [0,1,1]
Output: []
Explanation: The only possible triplet does not sum up to 0."#.to_string(),
                    r#"Input: nums = [0,0,0]
Output: [[0,0,0]]
Explanation: The only possible triplet sums up to 0."#.to_string(),
                ],
                function_signature: "var threeSum = function(nums)".to_string(),
            },
            Problem {
                id: 42,
                title: "Trapping Rain Water".to_string(),
                difficulty: "Hard".to_string(),
                description: r#"Given n non-negative integers representing an elevation map where the width of each bar is 1, compute how much water it can trap after raining.

Constraints:
- n == height.length
- 1 <= n <= 2 * 10^4
- 0 <= height[i] <= 10^5"#.to_string(),
                examples: vec![
                    r#"Input: height = [0,1,0,2,1,0,1,3,2,1,2,1]
Output: 6
Explanation: The above elevation map (black section) is represented by array [0,1,0,2,1,0,1,3,2,1,2,1]. In this case, 6 units of rain water (blue section) are being trapped."#.to_string(),
                    r#"Input: height = [4,2,0,3,2,5]
Output: 9"#.to_string(),
                ],
                function_signature: "var trap = function(height)".to_string(),
            },
        ])
    }

    pub fn get_problem(&self, id: u32) -> Result<Problem> {
        let problems = self.get_problems()?;
        problems
            .into_iter()
            .find(|p| p.id == id)
            .ok_or_else(|| anyhow::anyhow!("Problem not found"))
    }

    pub fn format_problem(&self, problem: &Problem) -> String {
        let mut output = String::new();

        output.push_str(&format!("Problem {}: {}\n", problem.id, problem.title));
        output.push_str(&format!("Difficulty: {}\n\n", problem.difficulty));
        output.push_str(&format!("{}\n\n", problem.description));

        if !problem.examples.is_empty() {
            output.push_str("Examples:\n\n");
            for (i, example) in problem.examples.iter().enumerate() {
                output.push_str(&format!("Example {}:\n{}\n\n", i + 1, example));
            }
        }

        output.push_str("\nKeyboard Shortcuts:\n");
        output.push_str("- Ctrl+R: Save & run tests\n");
        output.push_str("- Ctrl+Q: Switch focus between question and editor\n");
        output.push_str("- Ctrl+H: Back to home\n");
        output.push_str("- Ctrl+C: Quit application\n");
        output.push_str("- Esc: Close test results\n");
        output.push_str("- Up/Down: Scroll question (when focused)\n");

        output
    }

    /// Generate clean solution template for user to edit
    pub fn generate_boilerplate(&self, problem: &Problem) -> String {
        format!(
            r#"/**
 * Problem {id}: {title}
 * Difficulty: {difficulty}
 *
 * {description}
 */

{signature} {{
    // Your solution here
}};
"#,
            id = problem.id,
            title = problem.title,
            difficulty = problem.difficulty,
            description = problem.description.lines().next().unwrap_or(""),
            signature = problem.function_signature,
        )
    }

    /// Generate test runner that imports user's solution
    pub fn generate_test_runner(&self, problem: &Problem, solution_code: &str) -> String {
        let (func_name, test_cases) = self.get_problem_config(problem);

        format!(
            r#"// User's solution
{solution_code}

// ============== TEST RUNNER ==============
const testCases = {test_cases};

(function runTests() {{
    console.log('\n' + '='.repeat(50));
    console.log('Running tests for: {title}');
    console.log('='.repeat(50) + '\n');

    let passed = 0;
    let failed = 0;

    testCases.forEach((tc, i) => {{
        try {{
            const result = {func_name}(...tc.input);
            const resultStr = JSON.stringify(result);
            const expectedStr = JSON.stringify(tc.expected);

            const isEqual = resultStr === expectedStr;

            if (isEqual) {{
                console.log(`✓ Test ${{i + 1}}: PASSED`);
                passed++;
            }} else {{
                console.log(`✗ Test ${{i + 1}}: FAILED`);
                console.log(`  Input:    ${{JSON.stringify(tc.input)}}`);
                console.log(`  Expected: ${{expectedStr}}`);
                console.log(`  Got:      ${{resultStr}}`);
                failed++;
            }}
        }} catch (e) {{
            console.log(`✗ Test ${{i + 1}}: ERROR`);
            console.log(`  ${{e.message}}`);
            failed++;
        }}
    }});

    console.log('\n' + '-'.repeat(50));
    console.log(`Results: ${{passed}} passed, ${{failed}} failed out of ${{testCases.length}} tests`);
    console.log('-'.repeat(50) + '\n');

    if (failed === 0) {{
        console.log('🎉 All tests passed!');
    }}
}})();
"#,
            solution_code = solution_code,
            title = problem.title,
            func_name = func_name,
            test_cases = test_cases,
        )
    }

    fn get_problem_config(&self, problem: &Problem) -> (&'static str, &'static str) {
        match problem.id {
            1 => ("twoSum", r#"[
    { input: [[2, 7, 11, 15], 9], expected: [0, 1] },
    { input: [[3, 2, 4], 6], expected: [1, 2] },
    { input: [[3, 3], 6], expected: [0, 1] },
]"#),
            9 => ("isPalindrome", r#"[
    { input: [121], expected: true },
    { input: [-121], expected: false },
    { input: [10], expected: false },
]"#),
            13 => ("romanToInt", r#"[
    { input: ['III'], expected: 3 },
    { input: ['LVIII'], expected: 58 },
    { input: ['MCMXCIV'], expected: 1994 },
]"#),
            15 => ("threeSum", r#"[
    { input: [[-1, 0, 1, 2, -1, -4]], expected: [[-1, -1, 2], [-1, 0, 1]] },
    { input: [[0, 1, 1]], expected: [] },
    { input: [[0, 0, 0]], expected: [[0, 0, 0]] },
]"#),
            42 => ("trap", r#"[
    { input: [[0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]], expected: 6 },
    { input: [[4, 2, 0, 3, 2, 5]], expected: 9 },
]"#),
            _ => ("solution", r#"[
    // Add your test cases here
    // { input: [arg1, arg2], expected: result },
]"#),
        }
    }
}
