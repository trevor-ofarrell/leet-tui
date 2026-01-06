use anyhow::Result;
use rust_embed::Embed;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::language::{derive_signature, Language};

#[derive(Embed)]
#[folder = "problems/"]
#[include = "*.json"]
pub struct ProblemFiles;

#[derive(Embed)]
#[folder = "testcases/"]
#[include = "*.json"]
pub struct TestCaseFiles;

/// Extended test case file structure for Run vs Submit modes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtendedTestCases {
    pub problem_id: u32,
    pub title: String,
    pub run_tests: Vec<TestCase>,
    pub submit_tests: Vec<TestCase>,
}

/// Test mode: Run (quick 3-5 tests) or Submit (full 50-200 tests)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TestMode {
    #[default]
    Run,
    Submit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Problem {
    pub id: u32,
    pub title: String,
    pub difficulty: String,
    #[serde(default)]
    pub category: String,
    pub description: String,
    pub examples: Vec<String>,
    pub function_signature: String,
    pub function_name: String,
    pub test_cases: Vec<TestCase>,
    pub complexity_generator: String,
    #[serde(skip)]
    pub position: u32,
    #[serde(skip)]
    pub blind75: bool,
}

impl Problem {
    /// Get the function signature for a specific language
    pub fn get_signature(&self, lang: Language) -> String {
        derive_signature(&self.function_name, &self.function_signature, lang)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCase {
    pub input: serde_json::Value,
    pub expected: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct Blind75Order {
    order: Vec<OrderEntry>,
}

#[derive(Debug, Deserialize)]
struct OrderEntry {
    #[allow(dead_code)]
    position: u32,
    id: u32,
    category: String,
    #[serde(default)]
    blind75: bool,
}

pub struct LeetCodeClient {
    problems: Vec<Problem>,
    extended_tests: HashMap<u32, ExtendedTestCases>,
}

impl LeetCodeClient {
    pub fn new() -> Self {
        let mut problems_map: HashMap<u32, Problem> = HashMap::new();

        // Load all problem files
        for file in <ProblemFiles as Embed>::iter() {
            // Skip the order file
            if file.contains("blind75_order") {
                continue;
            }
            if let Some(content) = <ProblemFiles as Embed>::get(&file) {
                if let Ok(json_str) = std::str::from_utf8(&content.data) {
                    match serde_json::from_str::<Problem>(json_str) {
                        Ok(problem) => {
                            problems_map.insert(problem.id, problem);
                        }
                        Err(e) => eprintln!("Failed to parse {}: {}", file, e),
                    }
                }
            }
        }

        // Load the order file and sort problems accordingly
        let mut problems = Vec::new();
        if let Some(order_content) = <ProblemFiles as Embed>::get("blind75_order.json") {
            if let Ok(json_str) = std::str::from_utf8(&order_content.data) {
                if let Ok(order) = serde_json::from_str::<Blind75Order>(json_str) {
                    for entry in order.order {
                        if let Some(mut problem) = problems_map.remove(&entry.id) {
                            // Update category from order file if different
                            if problem.category.is_empty() || problem.category != entry.category {
                                problem.category = entry.category;
                            }
                            // Set position from the order file
                            problem.position = entry.position;
                            // Set blind75 flag from the order file
                            problem.blind75 = entry.blind75;
                            problems.push(problem);
                        }
                    }
                }
            }
        }

        // Add any remaining problems not in the order (shouldn't happen, but just in case)
        let mut remaining: Vec<Problem> = problems_map.into_values().collect();
        remaining.sort_by_key(|p| p.id);
        problems.extend(remaining);

        // Load extended test cases from testcases/ directory
        let mut extended_tests: HashMap<u32, ExtendedTestCases> = HashMap::new();
        for file in <TestCaseFiles as Embed>::iter() {
            if let Some(content) = <TestCaseFiles as Embed>::get(&file) {
                if let Ok(json_str) = std::str::from_utf8(&content.data) {
                    match serde_json::from_str::<ExtendedTestCases>(json_str) {
                        Ok(ext_tests) => {
                            extended_tests.insert(ext_tests.problem_id, ext_tests);
                        }
                        Err(e) => eprintln!("Failed to parse extended tests {}: {}", file, e),
                    }
                }
            }
        }

        Self { problems, extended_tests }
    }

    /// Get test cases for a problem based on test mode
    pub fn get_test_cases(&self, problem_id: u32, mode: TestMode) -> Vec<TestCase> {
        if let Some(ext_tests) = self.extended_tests.get(&problem_id) {
            match mode {
                TestMode::Run => ext_tests.run_tests.clone(),
                TestMode::Submit => ext_tests.submit_tests.clone(),
            }
        } else {
            // Fallback to problem's built-in test cases
            self.problems
                .iter()
                .find(|p| p.id == problem_id)
                .map(|p| p.test_cases.clone())
                .unwrap_or_default()
        }
    }

    pub fn get_problems(&self) -> Result<Vec<Problem>> {
        Ok(self.problems.clone())
    }

    pub fn format_problem(&self, problem: &Problem) -> String {
        let mut output = String::new();

        output.push_str(&format!("Problem {}: {}\n", problem.id, problem.title));
        output.push_str(&format!("Difficulty: {}\n", problem.difficulty));
        if !problem.category.is_empty() {
            output.push_str(&format!("Category: {}\n", problem.category));
        }
        output.push_str(&format!("\n{}\n\n", problem.description));

        if !problem.examples.is_empty() {
            output.push_str("Examples:\n\n");
            for (i, example) in problem.examples.iter().enumerate() {
                output.push_str(&format!("Example {}:\n{}\n\n", i + 1, example));
            }
        }

        output.push_str("\nKeyboard Shortcuts:\n");
        output.push_str("- Ctrl+R: Run tests (quick, 3-5 cases)\n");
        output.push_str("- Ctrl+S: Submit (full, 50-200 cases)\n");
        output.push_str("- Ctrl+Q: Switch focus between question and editor\n");
        output.push_str("- Ctrl+H: Back to home\n");
        output.push_str("- Ctrl+C: Quit application\n");
        output.push_str("- Esc: Close test results\n");
        output.push_str("- Up/Down: Scroll question (when focused)\n");

        output
    }

    /// Generate clean solution template for user to edit
    pub fn generate_boilerplate(&self, problem: &Problem, lang: Language) -> String {
        let signature = problem.get_signature(lang);
        let desc_first_line = problem.description.lines().next().unwrap_or("");

        match lang {
            Language::JavaScript => format!(
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
                description = desc_first_line,
                signature = problem.function_signature,
            ),

            Language::Python => format!(
                r##""""
Problem {id}: {title}
Difficulty: {difficulty}

{description}
"""

{signature}
    # Your solution here
    pass
"##,
                id = problem.id,
                title = problem.title,
                difficulty = problem.difficulty,
                description = desc_first_line,
                signature = signature,
            ),

            Language::C => format!(
                r#"/*
 * Problem {id}: {title}
 * Difficulty: {difficulty}
 *
 * {description}
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

// Helper for returning arrays (you may need to adjust based on problem)
// int* result = malloc(sizeof(int) * size);
// *returnSize = size;

{signature} {{
    // Your solution here
    return NULL;
}}
"#,
                id = problem.id,
                title = problem.title,
                difficulty = problem.difficulty,
                description = desc_first_line,
                signature = signature,
            ),

            Language::Cpp => format!(
                r#"/*
 * Problem {id}: {title}
 * Difficulty: {difficulty}
 *
 * {description}
 */

#include <vector>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <algorithm>
#include <queue>
#include <stack>

using namespace std;

class Solution {{
public:
    {signature} {{
        // Your solution here
    }}
}};
"#,
                id = problem.id,
                title = problem.title,
                difficulty = problem.difficulty,
                description = desc_first_line,
                signature = signature,
            ),
        }
    }

    /// Generate test runner for the specified language with a specific test mode
    pub fn generate_test_runner_with_mode(
        &self,
        problem: &Problem,
        solution_code: &str,
        lang: Language,
        mode: TestMode,
    ) -> String {
        // Get test cases based on mode
        let test_cases = self.get_test_cases(problem.id, mode);

        // Create a temporary problem with the appropriate test cases
        let mut test_problem = problem.clone();
        test_problem.test_cases = test_cases;

        let is_submit = mode == TestMode::Submit;

        match lang {
            Language::JavaScript => self.generate_js_test_runner(&test_problem, solution_code, is_submit),
            Language::Python => self.generate_python_test_runner(&test_problem, solution_code, is_submit),
            Language::C => self.generate_c_test_runner(&test_problem, solution_code, is_submit),
            Language::Cpp => self.generate_cpp_test_runner(&test_problem, solution_code, is_submit),
        }
    }

    /// Generate JavaScript test runner
    fn generate_js_test_runner(&self, problem: &Problem, solution_code: &str, is_submit: bool) -> String {
        let test_cases_json = serde_json::to_string(&problem.test_cases)
            .unwrap_or_else(|_| "[]".to_string());

        let mode_label = if is_submit { "SUBMIT" } else { "RUN" };

        // For run mode: verbose output with all details
        let test_loop_code = if !is_submit {
            r#"
    testCases.forEach((tc, i) => {
        try {
            const start = now();
            const result = func(...tc.input);
            const elapsed = now() - start;
            totalTime += elapsed;

            const resultStr = JSON.stringify(result);
            const expectedStr = JSON.stringify(tc.expected);
            const isEqual = resultStr === expectedStr;

            if (isEqual) {
                console.log(`Test ${i + 1}: ✓ PASSED [${formatTime(elapsed)}]`);
                console.log(`  Input:    ${JSON.stringify(tc.input)}`);
                console.log(`  Output:   ${resultStr}`);
                passed++;
            } else {
                console.log(`Test ${i + 1}: ✗ FAILED [${formatTime(elapsed)}]`);
                console.log(`  Input:    ${JSON.stringify(tc.input)}`);
                console.log(`  Expected: ${expectedStr}`);
                console.log(`  Got:      ${resultStr}`);
                failed++;
            }
        } catch (e) {
            console.log(`Test ${i + 1}: ERROR - ${e.message}`);
            failed++;
        }
    });"#.to_string()
        } else {
            // Submit mode: clean summary with categories
            r#"
    const failures = [];
    const categories = { basic: { passed: 0, total: 0 }, edge: { passed: 0, total: 0 }, standard: { passed: 0, total: 0 } };

    // Categorize and run tests
    testCases.forEach((tc, i) => {
        // Determine category based on input characteristics
        const inp = tc.input;
        let category = 'standard';
        if (i < 5) {
            category = 'basic';  // First 5 tests are basic examples
        } else if (Array.isArray(inp) && inp.length > 0) {
            const firstArg = inp[0];
            if (Array.isArray(firstArg)) {
                if (firstArg.length === 0 || firstArg.length === 1 || firstArg.length === 2) {
                    category = 'edge';
                }
            } else if (typeof firstArg === 'string' && (firstArg === '' || firstArg.length <= 2)) {
                category = 'edge';
            }
        }
        categories[category].total++;

        try {
            const start = now();
            const result = func(...tc.input);
            const elapsed = now() - start;
            totalTime += elapsed;

            const resultStr = JSON.stringify(result);
            const expectedStr = JSON.stringify(tc.expected);
            const isEqual = resultStr === expectedStr;

            if (isEqual) {
                passed++;
                categories[category].passed++;
            } else {
                failed++;
                failures.push({ num: i + 1, category, input: tc.input, expected: expectedStr, got: resultStr });
            }
        } catch (e) {
            failed++;
            failures.push({ num: i + 1, category, error: e.message });
        }
    });

    // Show category breakdown
    console.log('┌─────────────────┬────────┬─────────┐');
    console.log('│ Category        │ Passed │  Total  │');
    console.log('├─────────────────┼────────┼─────────┤');
    if (categories.basic.total > 0) {
        console.log(`│ Basic           │ ${String(categories.basic.passed).padStart(6)} │ ${String(categories.basic.total).padStart(7)} │`);
    }
    if (categories.edge.total > 0) {
        console.log(`│ Edge Cases      │ ${String(categories.edge.passed).padStart(6)} │ ${String(categories.edge.total).padStart(7)} │`);
    }
    if (categories.standard.total > 0) {
        console.log(`│ Standard        │ ${String(categories.standard.passed).padStart(6)} │ ${String(categories.standard.total).padStart(7)} │`);
    }
    console.log('└─────────────────┴────────┴─────────┘');

    // Show failure details (max 3)
    if (failures.length > 0) {
        console.log(`\n❌ ${failures.length} Failed Test(s):`);
        failures.slice(0, 3).forEach(f => {
            console.log(`\n  Test ${f.num} (${f.category}):`);
            if (f.error) {
                console.log(`    Error: ${f.error}`);
            } else {
                const inp = JSON.stringify(f.input);
                console.log(`    Input:    ${inp.slice(0, 60)}${inp.length > 60 ? '...' : ''}`);
                console.log(`    Expected: ${f.expected.slice(0, 60)}${f.expected.length > 60 ? '...' : ''}`);
                console.log(`    Got:      ${f.got.slice(0, 60)}${f.got.length > 60 ? '...' : ''}`);
            }
        });
        if (failures.length > 3) {
            console.log(`\n  ... and ${failures.length - 3} more failures`);
        }
    }"#.to_string()
        };

        // Complexity analysis code (only for submit mode)
        let complexity_code = if is_submit {
            format!(r#"
// Complexity analysis
async function runComplexityAnalysis() {{
    console.log('\n' + '═'.repeat(50));
    console.log('Complexity Analysis');
    console.log('═'.repeat(50));
    console.log('\nWarming up JIT...');

    // Input generator from problem
    {complexity_generator}

    const sizes = [10000, 100000, 1000000];
    const WARMUP_ITERATIONS = 100;
    const MIN_BENCH_TIME_MS = 200;
    const BENCHMARK_ROUNDS = 5;

    // Warmup with smaller inputs
    try {{
        for (let warmupN of [1000, 5000, 10000]) {{
            const warmupInput = generateInput(warmupN);
            for (let i = 0; i < WARMUP_ITERATIONS; i++) {{
                func(...warmupInput);
            }}
        }}
    }} catch (e) {{
        console.log('Warmup failed:', e.message);
        return;
    }}

    console.log('Running benchmarks (5 rounds each)...\n');
    console.log('┌─────────────┬──────────────────┬────────────┐');
    console.log('│      n      │   Median Time    │  Std Dev   │');
    console.log('├─────────────┼──────────────────┼────────────┤');

    const medianTimes = [];

    for (const n of sizes) {{
        try {{
            const inputData = generateInput(n);
            const roundTimes = [];
            let totalIterations = 0;

            for (let round = 0; round < BENCHMARK_ROUNDS; round++) {{
                let iterations = 0;
                let benchTime = 0;

                while (benchTime < MIN_BENCH_TIME_MS) {{
                    const start = now();
                    func(...inputData);
                    benchTime += now() - start;
                    iterations++;
                }}

                const avgTime = benchTime / iterations;
                roundTimes.push(avgTime);
                totalIterations += iterations;
            }}

            // Calculate median
            roundTimes.sort((a, b) => a - b);
            const medianTime = roundTimes[Math.floor(roundTimes.length / 2)];
            medianTimes.push(medianTime);

            // Calculate std dev
            const mean = roundTimes.reduce((a, b) => a + b, 0) / roundTimes.length;
            const variance = roundTimes.reduce((sum, t) => sum + Math.pow(t - mean, 2), 0) / roundTimes.length;
            const stdDev = Math.sqrt(variance);
            const relStdDev = ((stdDev / mean) * 100).toFixed(1) + '%';

            const nStr = n.toLocaleString().padStart(11);
            const timeStr = formatTime(medianTime).padStart(16);
            const stdStr = relStdDev.padStart(10);
            console.log(`│ ${{nStr}} │ ${{timeStr}} │ ${{stdStr}} │`);
        }} catch (e) {{
            const nStr = n.toLocaleString().padStart(11);
            console.log(`│ ${{nStr}} │ ${{' ERROR'.padStart(16)}} │            │`);
            medianTimes.push(0);
        }}
    }}

    console.log('└─────────────┴──────────────────┴────────────┘');

    // Calculate complexity using log-log slope
    if (medianTimes.length >= 2 && medianTimes[0] > 0 && medianTimes[1] > 0) {{
        const slopes = [];
        for (let i = 1; i < medianTimes.length; i++) {{
            if (medianTimes[i-1] > 0 && medianTimes[i] > 0) {{
                const logNRatio = Math.log(sizes[i] / sizes[i-1]);
                const logTRatio = Math.log(medianTimes[i] / medianTimes[i-1]);
                slopes.push(logTRatio / logNRatio);
            }}
        }}

        if (slopes.length > 0) {{
            const avgSlope = slopes.reduce((a, b) => a + b, 0) / slopes.length;
            let complexity;
            if (avgSlope < 0.15) complexity = 'O(1)';
            else if (avgSlope < 0.45) complexity = 'O(log n)';
            else if (avgSlope < 1.25) complexity = 'O(n)';
            else if (avgSlope < 1.65) complexity = 'O(n log n)';
            else if (avgSlope < 2.4) complexity = 'O(n²)';
            else if (avgSlope < 3.4) complexity = 'O(n³)';
            else complexity = 'O(2ⁿ) or worse';

            console.log(`\nTime Complexity:  ${{complexity}} (slope: ${{avgSlope.toFixed(2)}})`);
            console.log('Space Complexity: O(n) estimated');
            console.log('\nNote: Std Dev < 5% indicates stable measurements');
        }}
    }}
}}

runComplexityAnalysis();"#, complexity_generator = problem.complexity_generator)
        } else {
            String::new()
        };

        format!(
            r#"// User's solution
{solution_code}

// ============== TEST RUNNER ==============
const testCases = {test_cases};
const func = {func_name};

// High-resolution timer (works in both Bun and Node)
const now = typeof Bun !== 'undefined'
    ? () => Bun.nanoseconds() / 1e6
    : () => performance.now();

const formatTime = (ms) => {{
    if (ms < 1) return `${{(ms * 1000).toFixed(2)}} µs`;
    if (ms < 1000) return `${{ms.toFixed(2)}} ms`;
    return `${{(ms / 1000).toFixed(2)}} s`;
}};

(function runTests() {{
    console.log('\n' + '═'.repeat(50));
    console.log('[{mode_label}] Testing: {title}');
    console.log('═'.repeat(50) + '\n');

    let passed = 0;
    let failed = 0;
    let totalTime = 0;

    {test_loop_code}

    console.log('\n' + '─'.repeat(50));
    console.log(`Results: ${{passed}} passed, ${{failed}} failed of ${{testCases.length}} tests`);
    console.log(`Total time: ${{formatTime(totalTime)}}`);
    console.log('─'.repeat(50));

    if (failed === 0) {{
        console.log('\n✅ All tests passed!');
        {complexity_code}
    }}
}})();
"#,
            solution_code = solution_code,
            title = problem.title,
            mode_label = mode_label,
            func_name = problem.function_name,
            test_cases = test_cases_json,
            test_loop_code = test_loop_code,
            complexity_code = complexity_code,
        )
    }

    /// Generate Python test runner
    fn generate_python_test_runner(&self, problem: &Problem, solution_code: &str, is_submit: bool) -> String {
        let test_cases_json = serde_json::to_string(&problem.test_cases)
            .unwrap_or_else(|_| "[]".to_string());

        // Convert JS complexity generator to Python
        let python_generator = self.convert_js_generator_to_python(&problem.complexity_generator);

        let mode_label = if is_submit { "SUBMIT" } else { "RUN" };

        // For submit mode: table format with timing
        // For run mode: verbose output with all details
        let test_output_code = if is_submit {
            r#"test_num = str(i + 1).rjust(3)
            time_str = format_time(elapsed).rjust(12)
            if result_str == expected_str:
                print(f"│ {test_num}   │   ✓    │{time_str} │")
                passed += 1
            else:
                print(f"│ {test_num}   │   ✗    │{time_str} │")
                failures.append({
                    'num': i + 1,
                    'input': tc['input'],
                    'expected': expected_str,
                    'got': result_str
                })
                failed += 1"#
        } else {
            r#"if result_str == expected_str:
                print(f"Test {i + 1}: ✓ PASSED [{format_time(elapsed)}]")
                print(f"  Input:    {json.dumps(tc['input'])}")
                print(f"  Output:   {result_str}")
                passed += 1
            else:
                print(f"Test {i + 1}: ✗ FAILED [{format_time(elapsed)}]")
                print(f"  Input:    {json.dumps(tc['input'])}")
                print(f"  Expected: {expected_str}")
                print(f"  Got:      {result_str}")
                failed += 1"#
        };

        let test_setup_code = if is_submit {
            r#"# Table header
    print('┌───────┬────────┬──────────────┐')
    print('│ Test  │ Status │     Time     │')
    print('├───────┼────────┼──────────────┤')

    failures = []"#
        } else {
            ""
        };

        let test_footer_code = if is_submit {
            r#"print('└───────┴────────┴──────────────┘')

    # Show failure details
    if failures:
        print('\n❌ Failed Tests:')
        for f in failures:
            print(f"\nTest {f['num']}:")
            inp = json.dumps(f['input'])
            print(f"  Input:    {inp[:80]}{'...' if len(inp) > 80 else ''}")
            print(f"  Expected: {f['expected'][:80]}{'...' if len(f['expected']) > 80 else ''}")
            print(f"  Got:      {f['got'][:80]}{'...' if len(f['got']) > 80 else ''}")"#
        } else {
            ""
        };

        // Only include complexity analysis in submit mode
        let complexity_call = if is_submit {
            "run_complexity_analysis()"
        } else {
            "pass  # Complexity analysis skipped in RUN mode"
        };

        format!(
            r#"# User's solution
{solution_code}

# ============== TEST RUNNER ==============
import time
import json
import traceback
import random
import math

test_cases = json.loads('''{test_cases}''')

def format_time(ns):
    if ns < 1000:
        return f"{{ns:.2f}} ns"
    elif ns < 1_000_000:
        return f"{{ns / 1000:.2f}} us"
    elif ns < 1_000_000_000:
        return f"{{ns / 1_000_000:.2f}} ms"
    else:
        return f"{{ns / 1_000_000_000:.2f}} s"

def run_tests():
    print("\n" + "=" * 50)
    print("[{mode_label}] Testing: {title}")
    print("=" * 50 + "\n")

    passed = 0
    failed = 0
    total_time = 0

    {test_setup_code}

    for i, tc in enumerate(test_cases):
        try:
            start = time.perf_counter_ns()
            result = {func_name}(*tc["input"])
            elapsed = time.perf_counter_ns() - start
            total_time += elapsed

            result_str = json.dumps(result, sort_keys=True)
            expected_str = json.dumps(tc["expected"], sort_keys=True)

            {test_output_code}
        except Exception as e:
            test_num = str(i + 1).rjust(3)
            print(f"│ {{test_num}}   │  ERR   │              │")
            failed += 1

    {test_footer_code}

    print("\n" + "-" * 50)
    print(f"Results: {{passed}} passed, {{failed}} failed of {{len(test_cases)}} tests")
    print(f"Total time: {{format_time(total_time)}}")
    print("-" * 50)

    if failed == 0:
        print("\n✅ All tests passed!")
        {complexity_call}

def run_complexity_analysis():
    import statistics
    print("\n" + "=" * 50)
    print("Complexity Analysis")
    print("=" * 50 + "\n")

    {complexity_generator}

    # Benchmark configuration for stable results
    WARMUP_ITERATIONS = 500
    MIN_BENCH_TIME = 200_000_000  # 200ms in nanoseconds
    BENCHMARK_ROUNDS = 5
    sizes = [10000, 100000, 1000000]
    median_times = []

    # Aggressive warmup - critical for Python JIT (if using PyPy) or interpreter cache
    print("Warming up...")
    try:
        for warmup_n in [1000, 10000, 50000]:
            warmup_input = generate_input(warmup_n)
            for _ in range(WARMUP_ITERATIONS):
                {func_name}(*warmup_input)
    except:
        pass

    print("Running benchmarks (5 rounds each, taking median)...\n")
    print("+-------------+------------------+------------+--------------+")
    print("|      n      |   Median Time    |    Runs    |   Std Dev    |")
    print("+-------------+------------------+------------+--------------+")

    for n in sizes:
        try:
            input_data = generate_input(n)
            round_times = []
            total_iterations = 0

            for _ in range(BENCHMARK_ROUNDS):
                iterations = 0
                bench_time = 0

                while bench_time < MIN_BENCH_TIME:
                    start = time.perf_counter_ns()
                    {func_name}(*input_data)
                    bench_time += time.perf_counter_ns() - start
                    iterations += 1

                avg_time = bench_time / iterations
                round_times.append(avg_time)
                total_iterations += iterations

            # Calculate median (more stable than mean)
            median_time = statistics.median(round_times)
            median_times.append(median_time)

            # Calculate relative standard deviation
            mean_time = statistics.mean(round_times)
            if len(round_times) > 1:
                std_dev = statistics.stdev(round_times)
                rel_std_dev = f"{{(std_dev / mean_time * 100):.1f}}%"
            else:
                rel_std_dev = "N/A"

            n_str = str(n).rjust(11)
            time_str = format_time(median_time).rjust(16)
            runs_str = str(total_iterations).rjust(10)
            print(f"| {{n_str}} | {{time_str}} | {{runs_str}} | {{rel_std_dev.rjust(12)}} |")
        except Exception as e:
            n_str = str(n).rjust(11)
            print(f"| {{n_str}} | {{'ERROR'.rjust(16)}} |            |              |")
            median_times.append(0)

    print("+-------------+------------------+------------+--------------+")

    # Calculate complexity using log-log slope method
    if len(median_times) >= 2 and median_times[0] > 0 and median_times[1] > 0:
        slopes = []
        for i in range(1, len(median_times)):
            if median_times[i-1] > 0 and median_times[i] > 0:
                log_n_ratio = math.log(sizes[i] / sizes[i-1])
                log_t_ratio = math.log(median_times[i] / median_times[i-1])
                slopes.append(log_t_ratio / log_n_ratio)

        if slopes:
            avg_slope = sum(slopes) / len(slopes)
            if avg_slope < 0.15:
                complexity = "O(1)"
            elif avg_slope < 0.45:
                complexity = "O(log n)"
            elif avg_slope < 1.25:
                complexity = "O(n)"
            elif avg_slope < 1.65:
                complexity = "O(n log n)"
            elif avg_slope < 2.4:
                complexity = "O(n^2)"
            elif avg_slope < 3.4:
                complexity = "O(n^3)"
            else:
                complexity = "O(2^n) or worse"

            print(f"\nTime Complexity:  {{complexity}} (slope: {{avg_slope:.2f}})")
            print("Space Complexity: O(n) estimated")
            print("\nNote: Std Dev < 5% indicates stable results")

if __name__ == "__main__":
    run_tests()
"#,
            solution_code = solution_code,
            title = problem.title,
            mode_label = mode_label,
            func_name = problem.function_name,
            test_cases = test_cases_json,
            test_setup_code = test_setup_code,
            test_output_code = test_output_code,
            test_footer_code = test_footer_code,
            complexity_generator = python_generator,
            complexity_call = complexity_call,
        )
    }

    /// Generate C test runner
    fn generate_c_test_runner(&self, problem: &Problem, solution_code: &str, is_submit: bool) -> String {
        // Analyze test cases to infer types
        let (param_types, return_type) = self.infer_c_types(problem);

        // Generate test case code
        let test_case_code = self.generate_c_test_cases(problem, &param_types, &return_type, is_submit);

        let mode_label = if is_submit { "SUBMIT" } else { "RUN" };
        let total_tests = problem.test_cases.len();

        let table_header = if is_submit {
            r#"printf("┌───────┬────────┬──────────────┐\n");
    printf("│ Test  │ Status │     Time     │\n");
    printf("├───────┼────────┼──────────────┤\n");"#
        } else {
            ""
        };

        let table_footer = if is_submit {
            r#"printf("└───────┴────────┴──────────────┘\n");"#
        } else {
            ""
        };

        format!(
            r#"#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <stdbool.h>
#include <limits.h>
#include <math.h>

// User's solution
{solution_code}

// ============== TEST UTILITIES ==============
void print_int_array(int* arr, int size) {{
    printf("[");
    for (int i = 0; i < size; i++) {{
        if (i > 0) printf(",");
        printf("%d", arr[i]);
    }}
    printf("]");
}}

int arrays_equal(int* a, int size_a, int* b, int size_b) {{
    if (size_a != size_b) return 0;
    for (int i = 0; i < size_a; i++) {{
        if (a[i] != b[i]) return 0;
    }}
    return 1;
}}

void format_time(double ns, char* buf) {{
    if (ns < 1000) {{
        sprintf(buf, "%.2f ns", ns);
    }} else if (ns < 1000000) {{
        sprintf(buf, "%.2f us", ns / 1000);
    }} else if (ns < 1000000000) {{
        sprintf(buf, "%.2f ms", ns / 1000000);
    }} else {{
        sprintf(buf, "%.2f s", ns / 1000000000);
    }}
}}

double get_time_ns() {{
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return ts.tv_sec * 1e9 + ts.tv_nsec;
}}

// ============== TEST RUNNER ==============
int main() {{
    printf("\n==================================================\n");
    printf("[{mode_label}] Testing: {title}\n");
    printf("==================================================\n\n");

    int passed = 0;
    int total = 0;
    double total_time = 0;
    char time_buf[32];

    {table_header}

{test_case_code}

    {table_footer}

    printf("\n--------------------------------------------------\n");
    format_time(total_time, time_buf);
    printf("Results: %d passed, %d failed of {total_tests} tests\n", passed, total - passed);
    printf("Total time: %s\n", time_buf);
    printf("--------------------------------------------------\n");

    if (passed == total) {{
        printf("\n✅ All tests passed!\n");
    }}

    return passed == total ? 0 : 1;
}}
"#,
            solution_code = solution_code,
            title = problem.title,
            mode_label = mode_label,
            table_header = table_header,
            test_case_code = test_case_code,
            table_footer = table_footer,
            total_tests = total_tests,
        )
    }

    /// Infer C types from test cases
    fn infer_c_types(&self, problem: &Problem) -> (Vec<String>, String) {
        if problem.test_cases.is_empty() {
            return (vec!["void*".to_string()], "void*".to_string());
        }

        let first_tc = &problem.test_cases[0];

        // Infer parameter types from input (input is an array of arguments)
        let param_types: Vec<String> = first_tc.input.as_array()
            .map(|arr| arr.iter().map(|v| self.json_to_c_type(v)).collect())
            .unwrap_or_else(|| vec!["void*".to_string()]);

        // Infer return type from expected
        let return_type = self.json_to_c_type(&first_tc.expected);

        (param_types, return_type)
    }

    /// Convert JSON value to C type string
    fn json_to_c_type(&self, value: &serde_json::Value) -> String {
        match value {
            serde_json::Value::Bool(_) => "bool".to_string(),
            serde_json::Value::Number(n) => {
                if n.is_i64() || n.is_u64() { "int".to_string() }
                else { "double".to_string() }
            }
            serde_json::Value::String(_) => "char*".to_string(),
            serde_json::Value::Array(_) => "int*".to_string(), // Simplified - assumes int arrays
            _ => "void*".to_string(),
        }
    }

    /// Convert JSON value to C literal (for arrays, generates array initializer)
    fn json_to_c_literal(&self, value: &serde_json::Value) -> String {
        match value {
            serde_json::Value::Bool(b) => if *b { "true" } else { "false" }.to_string(),
            serde_json::Value::Number(n) => n.to_string(),
            serde_json::Value::String(s) => format!("\"{}\"", s.replace("\"", "\\\"")),
            serde_json::Value::Array(arr) => {
                let elements: Vec<String> = arr.iter()
                    .map(|v| self.json_to_c_literal(v))
                    .collect();
                format!("{{{}}}", elements.join(", "))
            }
            serde_json::Value::Null => "NULL".to_string(),
            _ => "NULL".to_string(),
        }
    }

    /// Get array size from JSON value
    fn json_array_size(&self, value: &serde_json::Value) -> usize {
        match value {
            serde_json::Value::Array(arr) => arr.len(),
            _ => 0,
        }
    }

    /// Generate C test case code
    fn generate_c_test_cases(&self, problem: &Problem, param_types: &[String], return_type: &str, is_submit: bool) -> String {
        let mut code = String::new();
        let func_name = &problem.function_name;

        for (i, tc) in problem.test_cases.iter().enumerate() {
            code.push_str("    {\n");
            code.push_str("        total++;\n");

            // Generate input variables
            let mut arg_exprs = Vec::new();
            let inputs = tc.input.as_array().cloned().unwrap_or_default();
            for (j, (input_val, param_type)) in inputs.iter().zip(param_types.iter()).enumerate() {
                let var_name = format!("input{}", j);
                let literal = self.json_to_c_literal(input_val);

                if param_type == "int*" {
                    let size = self.json_array_size(input_val);
                    code.push_str(&format!("        int {}[] = {};\n", var_name, literal));
                    code.push_str(&format!("        int {}Size = {};\n", var_name, size));
                    arg_exprs.push(var_name.clone());
                    arg_exprs.push(format!("{}Size", var_name));
                } else {
                    code.push_str(&format!("        {} {} = {};\n", param_type, var_name, literal));
                    arg_exprs.push(var_name);
                }
            }

            // Generate expected
            if return_type == "int*" {
                let literal = self.json_to_c_literal(&tc.expected);
                let size = self.json_array_size(&tc.expected);
                code.push_str(&format!("        int expected[] = {};\n", literal));
                code.push_str(&format!("        int expected_size = {};\n", size));
            } else {
                let literal = self.json_to_c_literal(&tc.expected);
                code.push_str(&format!("        {} expected = {};\n", return_type, literal));
            }

            // Call function with timing
            code.push_str("        double start = get_time_ns();\n");
            if return_type == "int*" {
                code.push_str("        int returnSize;\n");
                code.push_str(&format!("        int* result = {}({}, &returnSize);\n", func_name, arg_exprs.join(", ")));
            } else {
                code.push_str(&format!("        {} result = {}({});\n", return_type, func_name, arg_exprs.join(", ")));
            }
            code.push_str("        double elapsed = get_time_ns() - start;\n");
            code.push_str("        total_time += elapsed;\n");

            // Compare results
            if return_type == "int*" {
                code.push_str("        int pass = arrays_equal(result, returnSize, expected, expected_size);\n");
            } else {
                code.push_str("        int pass = (result == expected);\n");
            }

            if is_submit {
                // Table output for submit mode
                code.push_str("        format_time(elapsed, time_buf);\n");
                code.push_str("        if (pass) {\n");
                code.push_str("            printf(\"│ %3d   │   ✓    │%12s │\\n\", total, time_buf);\n");
                code.push_str("            passed++;\n");
                code.push_str("        } else {\n");
                code.push_str("            printf(\"│ %3d   │   ✗    │%12s │\\n\", total, time_buf);\n");
                code.push_str("        }\n");
            } else {
                // Verbose output for run mode
                code.push_str("        format_time(elapsed, time_buf);\n");
                code.push_str("        if (pass) {\n");
                code.push_str(&format!("            printf(\"Test {}: ✓ PASSED [%s]\\n\", time_buf);\n", i + 1));
                code.push_str("            passed++;\n");
                code.push_str("        } else {\n");
                code.push_str(&format!("            printf(\"Test {}: ✗ FAILED [%s]\\n\", time_buf);\n", i + 1));
                if return_type == "bool" {
                    code.push_str("            printf(\"  Expected: %s\\n\", expected ? \"true\" : \"false\");\n");
                    code.push_str("            printf(\"  Got:      %s\\n\", result ? \"true\" : \"false\");\n");
                } else if return_type == "int" {
                    code.push_str("            printf(\"  Expected: %d\\n\", expected);\n");
                    code.push_str("            printf(\"  Got:      %d\\n\", result);\n");
                } else if return_type == "int*" {
                    code.push_str("            printf(\"  Expected: \"); print_int_array(expected, expected_size); printf(\"\\n\");\n");
                    code.push_str("            printf(\"  Got:      \"); print_int_array(result, returnSize); printf(\"\\n\");\n");
                }
                code.push_str("        }\n");
            }

            // Free allocated memory for array returns
            if return_type == "int*" {
                code.push_str("        if (result) free(result);\n");
            }

            code.push_str("    }\n\n");
        }

        code
    }

    /// Generate C++ test runner
    fn generate_cpp_test_runner(&self, problem: &Problem, solution_code: &str, is_submit: bool) -> String {
        // Analyze test cases to infer types
        let (param_types, return_type) = self.infer_cpp_types(problem);

        // Generate test case code
        let test_case_code = self.generate_cpp_test_cases(problem, &param_types, &return_type, is_submit);

        let mode_label = if is_submit { "SUBMIT" } else { "RUN" };
        let total_tests = problem.test_cases.len();

        let table_header = if is_submit {
            r#"cout << "┌───────┬────────┬──────────────┐" << endl;
    cout << "│ Test  │ Status │     Time     │" << endl;
    cout << "├───────┼────────┼──────────────┤" << endl;"#
        } else {
            ""
        };

        let table_footer = if is_submit {
            r#"cout << "└───────┴────────┴──────────────┘" << endl;"#
        } else {
            ""
        };

        format!(
            r#"#include <iostream>
#include <vector>
#include <string>
#include <unordered_set>
#include <unordered_map>
#include <algorithm>
#include <chrono>
#include <sstream>
#include <climits>
#include <iomanip>

using namespace std;

// User's solution
{solution_code}

// ============== TEST UTILITIES ==============
template<typename T>
string vec_to_string(const vector<T>& v) {{
    stringstream ss;
    ss << "[";
    for (size_t i = 0; i < v.size(); i++) {{
        if (i > 0) ss << ",";
        ss << v[i];
    }}
    ss << "]";
    return ss.str();
}}

template<typename T>
string vec2d_to_string(const vector<vector<T>>& v) {{
    stringstream ss;
    ss << "[";
    for (size_t i = 0; i < v.size(); i++) {{
        if (i > 0) ss << ",";
        ss << vec_to_string(v[i]);
    }}
    ss << "]";
    return ss.str();
}}

string vec_str_to_string(const vector<string>& v) {{
    stringstream ss;
    ss << "[";
    for (size_t i = 0; i < v.size(); i++) {{
        if (i > 0) ss << ",";
        ss << "\"" << v[i] << "\"";
    }}
    ss << "]";
    return ss.str();
}}

// Compare vectors (order-sensitive)
template<typename T>
bool vec_equal(vector<T> a, vector<T> b) {{
    return a == b;
}}

// Compare vectors (order-insensitive - for problems that say "return in any order")
template<typename T>
bool vec_equal_any_order(vector<T> a, vector<T> b) {{
    sort(a.begin(), a.end());
    sort(b.begin(), b.end());
    return a == b;
}}

// Compare 2D vectors (order-insensitive for both inner and outer)
template<typename T>
bool vec2d_equal_any_order(vector<vector<T>> a, vector<vector<T>> b) {{
    if (a.size() != b.size()) return false;
    // Sort each inner vector
    for (auto& v : a) sort(v.begin(), v.end());
    for (auto& v : b) sort(v.begin(), v.end());
    // Sort outer vectors
    sort(a.begin(), a.end());
    sort(b.begin(), b.end());
    return a == b;
}}

string format_time(double ns) {{
    stringstream ss;
    if (ns < 1000) {{
        ss << fixed << setprecision(2) << ns << " ns";
    }} else if (ns < 1000000) {{
        ss << fixed << setprecision(2) << (ns / 1000) << " us";
    }} else if (ns < 1000000000) {{
        ss << fixed << setprecision(2) << (ns / 1000000) << " ms";
    }} else {{
        ss << fixed << setprecision(2) << (ns / 1000000000) << " s";
    }}
    return ss.str();
}}

// ============== TEST RUNNER ==============
int main() {{
    cout << "\n==================================================" << endl;
    cout << "[{mode_label}] Testing: {title}" << endl;
    cout << "==================================================" << endl << endl;

    Solution sol;
    int passed = 0;
    int total = 0;
    double total_time = 0;

    {table_header}

{test_case_code}

    {table_footer}

    cout << "\n--------------------------------------------------" << endl;
    cout << "Results: " << passed << " passed, " << (total - passed) << " failed of {total_tests} tests" << endl;
    cout << "Total time: " << format_time(total_time) << endl;
    cout << "--------------------------------------------------" << endl;

    if (passed == total) {{
        cout << "\n✅ All tests passed!" << endl;
    }}

    return passed == total ? 0 : 1;
}}
"#,
            solution_code = solution_code,
            title = problem.title,
            mode_label = mode_label,
            table_header = table_header,
            test_case_code = test_case_code,
            table_footer = table_footer,
            total_tests = total_tests,
        )
    }

    /// Infer C++ types from test cases
    fn infer_cpp_types(&self, problem: &Problem) -> (Vec<String>, String) {
        if problem.test_cases.is_empty() {
            return (vec!["auto".to_string()], "auto".to_string());
        }

        let first_tc = &problem.test_cases[0];

        // Infer parameter types from input (input is an array of arguments)
        let param_types: Vec<String> = first_tc.input.as_array()
            .map(|arr| arr.iter().map(|v| self.json_to_cpp_type(v)).collect())
            .unwrap_or_else(|| vec!["auto".to_string()]);

        // Infer return type from expected
        let return_type = self.json_to_cpp_type(&first_tc.expected);

        (param_types, return_type)
    }

    /// Convert JSON value to C++ type string
    fn json_to_cpp_type(&self, value: &serde_json::Value) -> String {
        match value {
            serde_json::Value::Bool(_) => "bool".to_string(),
            serde_json::Value::Number(n) => {
                if n.is_i64() || n.is_u64() { "int".to_string() }
                else { "double".to_string() }
            }
            serde_json::Value::String(_) => "string".to_string(),
            serde_json::Value::Array(arr) => {
                if arr.is_empty() {
                    "vector<int>".to_string() // Default assumption
                } else {
                    let inner_type = self.json_to_cpp_type(&arr[0]);
                    format!("vector<{}>", inner_type)
                }
            }
            _ => "auto".to_string(),
        }
    }

    /// Convert JSON value to C++ literal
    fn json_to_cpp_literal(&self, value: &serde_json::Value) -> String {
        match value {
            serde_json::Value::Bool(b) => if *b { "true" } else { "false" }.to_string(),
            serde_json::Value::Number(n) => n.to_string(),
            serde_json::Value::String(s) => format!("\"{}\"", s.replace("\"", "\\\"")),
            serde_json::Value::Array(arr) => {
                let elements: Vec<String> = arr.iter()
                    .map(|v| self.json_to_cpp_literal(v))
                    .collect();
                format!("{{{}}}", elements.join(", "))
            }
            serde_json::Value::Null => "nullptr".to_string(),
            _ => "{}".to_string(),
        }
    }

    /// Generate C++ test case code
    fn generate_cpp_test_cases(&self, problem: &Problem, param_types: &[String], return_type: &str, is_submit: bool) -> String {
        let mut code = String::new();
        let func_name = &problem.function_name;

        // Determine if we need order-insensitive comparison
        let any_order = problem.description.to_lowercase().contains("any order");

        for (i, tc) in problem.test_cases.iter().enumerate() {
            code.push_str("    {\n");
            code.push_str("        total++;\n");

            // Generate input variables
            let mut arg_names = Vec::new();
            let inputs = tc.input.as_array().cloned().unwrap_or_default();
            for (j, (input_val, param_type)) in inputs.iter().zip(param_types.iter()).enumerate() {
                let var_name = format!("input{}", j);
                let literal = self.json_to_cpp_literal(input_val);
                code.push_str(&format!("        {} {} = {};\n", param_type, var_name, literal));
                arg_names.push(var_name);
            }

            // Generate expected
            let expected_literal = self.json_to_cpp_literal(&tc.expected);
            code.push_str(&format!("        {} expected = {};\n", return_type, expected_literal));

            // Call function with timing
            code.push_str("        auto start = chrono::high_resolution_clock::now();\n");
            code.push_str(&format!("        auto result = sol.{}({});\n", func_name, arg_names.join(", ")));
            code.push_str("        auto end = chrono::high_resolution_clock::now();\n");
            code.push_str("        double elapsed = chrono::duration<double, nano>(end - start).count();\n");
            code.push_str("        total_time += elapsed;\n");

            // Compare results
            let compare_expr = if return_type.starts_with("vector<") {
                if any_order {
                    "vec_equal_any_order(result, expected)".to_string()
                } else {
                    "vec_equal(result, expected)".to_string()
                }
            } else {
                "result == expected".to_string()
            };

            code.push_str(&format!("        bool pass = {};\n", compare_expr));

            if is_submit {
                // Table output for submit mode
                code.push_str("        string time_str = format_time(elapsed);\n");
                code.push_str("        if (pass) {\n");
                code.push_str("            cout << \"│ \" << setw(3) << total << \"   │   ✓    │\" << setw(12) << time_str << \" │\" << endl;\n");
                code.push_str("            passed++;\n");
                code.push_str("        } else {\n");
                code.push_str("            cout << \"│ \" << setw(3) << total << \"   │   ✗    │\" << setw(12) << time_str << \" │\" << endl;\n");
                code.push_str("        }\n");
            } else {
                // Verbose output for run mode
                // Result to string for printing
                let result_to_str = if return_type == "bool" {
                    "(result ? \"true\" : \"false\")".to_string()
                } else if return_type == "int" || return_type == "double" {
                    "to_string(result)".to_string()
                } else if return_type == "string" {
                    "result".to_string()
                } else if return_type.starts_with("vector<vector<") {
                    "vec2d_to_string(result)".to_string()
                } else if return_type == "vector<string>" {
                    "vec_str_to_string(result)".to_string()
                } else if return_type.starts_with("vector<") {
                    "vec_to_string(result)".to_string()
                } else {
                    "\"[complex type]\"".to_string()
                };

                let expected_to_str = if return_type == "bool" {
                    "(expected ? \"true\" : \"false\")".to_string()
                } else if return_type == "int" || return_type == "double" {
                    "to_string(expected)".to_string()
                } else if return_type == "string" {
                    "expected".to_string()
                } else if return_type.starts_with("vector<vector<") {
                    "vec2d_to_string(expected)".to_string()
                } else if return_type == "vector<string>" {
                    "vec_str_to_string(expected)".to_string()
                } else if return_type.starts_with("vector<") {
                    "vec_to_string(expected)".to_string()
                } else {
                    "\"[complex type]\"".to_string()
                };

                code.push_str("        if (pass) {\n");
                code.push_str(&format!("            cout << \"Test {}: ✓ PASSED [\" << format_time(elapsed) << \"]\" << endl;\n", i + 1));
                code.push_str("            passed++;\n");
                code.push_str("        } else {\n");
                code.push_str(&format!("            cout << \"Test {}: ✗ FAILED [\" << format_time(elapsed) << \"]\" << endl;\n", i + 1));
                code.push_str(&format!("            cout << \"  Expected: \" << {} << endl;\n", expected_to_str));
                code.push_str(&format!("            cout << \"  Got:      \" << {} << endl;\n", result_to_str));
                code.push_str("        }\n");
            }
            code.push_str("    }\n\n");
        }

        code
    }

    /// Convert JavaScript complexity generator to Python
    fn convert_js_generator_to_python(&self, js_code: &str) -> String {
        // Basic conversion from JS to Python syntax
        let python_code = js_code
            // Convert function declaration
            .replace("const generateInput = (n) => {", "def generate_input(n):")
            .replace("const generateInput = (n) =>", "def generate_input(n):")
            // Convert common Array.from patterns BEFORE removing semicolons
            // Pattern: Array.from({length: n}, (_, i) => i) -> list(range(n))
            .replace("Array.from({length: n}, (_, i) => i)", "list(range(n))")
            .replace("Array.from({length: n}, (_,i) => i)", "list(range(n))")
            // Pattern: Array.from({length: n}, () => 0) -> [0] * n
            .replace("Array.from({length: n}, () => 0)", "[0] * n")
            // Pattern with random: Array.from({length: n}, () => Math.floor(Math.random() * X))
            .replace("Array.from({length: n}, () => Math.floor(Math.random() * 10000))", "[int(random.random() * 10000) for _ in range(n)]")
            .replace("Array.from({length: n}, () => Math.floor(Math.random() * 26))", "[int(random.random() * 26) for _ in range(n)]")
            .replace("Array.from({length: n}, () => Math.floor(Math.random() * 100))", "[int(random.random() * 100) for _ in range(n)]")
            .replace("Array.from({length: n}, () => Math.floor(Math.random() * n))", "[int(random.random() * n) for _ in range(n)]")
            .replace("Array.from({length: n}, () => Math.floor(Math.random() * 2147483647))", "[int(random.random() * 2147483647) for _ in range(n)]")
            // Remove trailing semicolons and braces
            .replace("};", "")
            .replace(";", "")
            // Convert math functions
            .replace("Math.floor", "int")
            .replace("Math.random()", "random.random()")
            .replace("Math.min", "min")
            .replace("Math.max", "max")
            .replace("Math.sqrt", "math.sqrt")
            // Convert const/let/var
            .replace("const ", "")
            .replace("let ", "")
            .replace("var ", "")
            // Convert for loops
            .replace("for(let i=0; i<n; i++)", "for i in range(n):")
            .replace("for (let i=0; i<n; i++)", "for i in range(n):")
            .replace("for(let i = 0; i < n; i++)", "for i in range(n):")
            .replace("for (let i = 0; i < n; i++)", "for i in range(n):");

        // If conversion seems incomplete, provide a fallback
        if !python_code.contains("def generate_input") {
            return r#"def generate_input(n):
    # Auto-generated - may need manual adjustment
    return [list(range(n)), n // 2]"#
                .to_string();
        }

        // Fix Python indentation:
        // - The template has 4-space indent before {complexity_generator}
        // - Multi-line substitution only indents the first line
        // - So we need to add 4-space indent to lines 2+ ourselves
        // - Function body needs additional 4-space indent (8 total after first line)
        let mut result = String::new();
        let mut in_function = false;
        let mut first_line = true;

        for line in python_code.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            if trimmed.starts_with("def generate_input") {
                if !first_line {
                    result.push_str("    "); // Template indent for continuation lines
                }
                result.push_str(trimmed);
                result.push('\n');
                in_function = true;
                first_line = false;
            } else if in_function {
                result.push_str("        "); // 4 (template) + 4 (function body) = 8 spaces
                result.push_str(trimmed);
                result.push('\n');
            }
        }

        result
    }
}
