use anyhow::Result;
use rust_embed::Embed;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::language::{derive_signature, Language};

#[derive(Embed)]
#[folder = "problems/"]
#[include = "*.json"]
pub struct ProblemFiles;

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

        Self { problems }
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
        output.push_str("- Ctrl+R: Save & run tests\n");
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

    /// Generate test runner for the specified language
    pub fn generate_test_runner(
        &self,
        problem: &Problem,
        solution_code: &str,
        lang: Language,
    ) -> String {
        match lang {
            Language::JavaScript => self.generate_js_test_runner(problem, solution_code),
            Language::Python => self.generate_python_test_runner(problem, solution_code),
            Language::C => self.generate_c_test_runner(problem, solution_code),
            Language::Cpp => self.generate_cpp_test_runner(problem, solution_code),
        }
    }

    /// Generate JavaScript test runner
    fn generate_js_test_runner(&self, problem: &Problem, solution_code: &str) -> String {
        let test_cases_json = serde_json::to_string(&problem.test_cases)
            .unwrap_or_else(|_| "[]".to_string());

        format!(
            r#"// User's solution
{solution_code}

// ============== TEST RUNNER ==============
const testCases = {test_cases};

// High-resolution timer (works in both Bun and Node)
const now = typeof Bun !== 'undefined'
    ? () => Bun.nanoseconds() / 1e6
    : () => performance.now();

// Memory usage (works in both Bun and Node)
const getMemory = () => {{
    if (typeof Bun !== 'undefined') {{
        return process.memoryUsage().heapUsed;
    }}
    if (typeof process !== 'undefined') {{
        return process.memoryUsage().heapUsed;
    }}
    return 0;
}};

const formatTime = (ms) => {{
    if (ms < 1) return `${{(ms * 1000).toFixed(2)}} microseconds`;
    if (ms < 1000) return `${{ms.toFixed(2)}} milliseconds`;
    return `${{(ms / 1000).toFixed(2)}} seconds`;
}};

const formatMemory = (bytes) => {{
    if (bytes < 1024) return `${{bytes}}B`;
    if (bytes < 1024 * 1024) return `${{(bytes / 1024).toFixed(1)}}KB`;
    return `${{(bytes / (1024 * 1024)).toFixed(2)}}MB`;
}};

(function runTests() {{
    console.log('\n' + '='.repeat(50));
    console.log('Running tests for: {title}');
    console.log('='.repeat(50) + '\n');

    let passed = 0;
    let failed = 0;
    let totalTime = 0;

    testCases.forEach((tc, i) => {{
        try {{
            const startMem = getMemory();
            const start = now();
            const result = {func_name}(...tc.input);
            const elapsed = now() - start;
            const memUsed = getMemory() - startMem;
            totalTime += elapsed;

            const resultStr = JSON.stringify(result);
            const expectedStr = JSON.stringify(tc.expected);
            const isEqual = resultStr === expectedStr;

            if (isEqual) {{
                console.log(`Test ${{i + 1}}: PASSED [${{formatTime(elapsed)}}]`);
                console.log(`  Input:    ${{JSON.stringify(tc.input)}}`);
                console.log(`  Output:   ${{resultStr}}`);
                passed++;
            }} else {{
                console.log(`Test ${{i + 1}}: FAILED [${{formatTime(elapsed)}}]`);
                console.log(`  Input:    ${{JSON.stringify(tc.input)}}`);
                console.log(`  Expected: ${{expectedStr}}`);
                console.log(`  Got:      ${{resultStr}}`);
                failed++;
            }}
        }} catch (e) {{
            console.log(`Test ${{i + 1}}: ERROR`);
            console.log(`  ${{e.message}}`);
            failed++;
        }}
    }});

    console.log('\n' + '-'.repeat(50));
    console.log(`Results: ${{passed}} passed, ${{failed}} failed out of ${{testCases.length}} tests`);
    console.log(`Total time: ${{formatTime(totalTime)}}`);
    console.log('-'.repeat(50));

    if (failed === 0) {{
        console.log('\nAll tests passed!');

        // Run complexity analysis using the doubling hypothesis
        console.log('\n' + '='.repeat(50));
        console.log('Complexity Analysis (Doubling Method)');
        console.log('='.repeat(50) + '\n');

        {complexity_generator}

        // Force garbage collection if available (Bun)
        const gc = () => {{ if (typeof Bun !== 'undefined') Bun.gc(true); }};

        // Use orders of magnitude for accurate complexity detection
        const sizes = [10000, 100000, 1000000];
        const times = [];
        const memories = [];
        const MIN_BENCH_TIME = 50; // Run each size for at least 50ms total

        // Global warmup - ensure JIT is fully optimized before measuring
        const warmupInput = generateInput(50000);
        for (let i = 0; i < 50; i++) {{
            try {{ {func_name}(...warmupInput); }} catch {{}}
        }}
        gc();

        sizes.forEach(n => {{
            gc();
            const input = generateInput(n);
            gc();

            // Run until we have at least MIN_BENCH_TIME ms of total execution
            let iterations = 0;
            let totalTime = 0;
            const benchStart = now();

            while (totalTime < MIN_BENCH_TIME) {{
                const start = now();
                try {{ {func_name}(...input); }} catch {{}}
                totalTime = now() - benchStart;
                iterations++;
            }}

            const avgTime = totalTime / iterations;
            const memAfter = getMemory();
            times.push(avgTime);
            memories.push(memAfter);

            console.log(`n=${{String(n).padStart(7)}}: ${{formatTime(avgTime).padStart(18)}} (${{iterations}} runs)`);
        }});

        // Calculate growth rate using log-log slope method
        // For O(n^k), log(time) = k * log(n) + c, so slope = k
        if (times.length >= 2 && times[0] > 0) {{
            const slopes = [];
            for (let i = 1; i < times.length; i++) {{
                if (times[i-1] > 0 && times[i] > 0) {{
                    const logNRatio = Math.log(sizes[i] / sizes[i-1]);
                    const logTRatio = Math.log(times[i] / times[i-1]);
                    slopes.push(logTRatio / logNRatio);
                }}
            }}

            if (slopes.length >= 1) {{
                const avgSlope = slopes.reduce((a,b) => a+b, 0) / slopes.length;

                let timeComplexity = '';
                if (avgSlope < 0.2) timeComplexity = 'O(1)';
                else if (avgSlope < 0.5) timeComplexity = 'O(log n)';
                else if (avgSlope < 1.3) timeComplexity = 'O(n)';
                else if (avgSlope < 1.7) timeComplexity = 'O(n log n)';
                else if (avgSlope < 2.5) timeComplexity = 'O(n^2)';
                else if (avgSlope < 3.5) timeComplexity = 'O(n^3)';
                else timeComplexity = 'O(2^n) or worse';

                console.log(`\nTime Complexity: ${{timeComplexity}} (slope: ${{avgSlope.toFixed(2)}})`);
                console.log('Reference: O(1)=0  O(n)=1  O(n^2)=2  O(n^3)=3');
            }}
        }}
    }}
}})();
"#,
            solution_code = solution_code,
            title = problem.title,
            func_name = problem.function_name,
            test_cases = test_cases_json,
            complexity_generator = problem.complexity_generator,
        )
    }

    /// Generate Python test runner
    fn generate_python_test_runner(&self, problem: &Problem, solution_code: &str) -> String {
        let test_cases_json = serde_json::to_string(&problem.test_cases)
            .unwrap_or_else(|_| "[]".to_string());

        // Convert JS complexity generator to Python
        let python_generator = self.convert_js_generator_to_python(&problem.complexity_generator);

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
    print("Running tests for: {title}")
    print("=" * 50 + "\n")

    passed = 0
    failed = 0
    total_time = 0

    for i, tc in enumerate(test_cases):
        try:
            start = time.perf_counter_ns()
            result = {func_name}(*tc["input"])
            elapsed = time.perf_counter_ns() - start
            total_time += elapsed

            result_str = json.dumps(result, sort_keys=True)
            expected_str = json.dumps(tc["expected"], sort_keys=True)

            if result_str == expected_str:
                print(f"Test {{i + 1}}: PASSED [{{format_time(elapsed)}}]")
                print(f"  Input:    {{json.dumps(tc['input'])}}")
                print(f"  Output:   {{result_str}}")
                passed += 1
            else:
                print(f"Test {{i + 1}}: FAILED [{{format_time(elapsed)}}]")
                print(f"  Input:    {{json.dumps(tc['input'])}}")
                print(f"  Expected: {{expected_str}}")
                print(f"  Got:      {{result_str}}")
                failed += 1
        except Exception as e:
            print(f"Test {{i + 1}}: ERROR")
            print(f"  {{str(e)}}")
            traceback.print_exc()
            failed += 1

    print("\n" + "-" * 50)
    print(f"Results: {{passed}} passed, {{failed}} failed out of {{len(test_cases)}} tests")
    print(f"Total time: {{format_time(total_time)}}")
    print("-" * 50)

    if failed == 0:
        print("\nAll tests passed!")
        run_complexity_analysis()

def run_complexity_analysis():
    print("\n" + "=" * 50)
    print("Complexity Analysis (Doubling Method)")
    print("=" * 50 + "\n")

    {complexity_generator}

    sizes = [10000, 100000, 1000000]
    times = []

    # Warmup
    try:
        warmup_input = generate_input(50000)
        for _ in range(10):
            {func_name}(*warmup_input)
    except:
        pass

    for n in sizes:
        try:
            input_data = generate_input(n)
            iterations = 0
            total_time = 0
            min_bench_time = 50_000_000  # 50ms in nanoseconds

            while total_time < min_bench_time:
                start = time.perf_counter_ns()
                {func_name}(*input_data)
                total_time += time.perf_counter_ns() - start
                iterations += 1

            avg_time = total_time / iterations
            times.append(avg_time)
            print(f"n={{str(n).rjust(7)}}: {{format_time(avg_time).rjust(18)}} ({{iterations}} runs)")
        except Exception as e:
            print(f"n={{str(n).rjust(7)}}: ERROR - {{e}}")
            times.append(0)

    # Calculate complexity
    if len(times) >= 2 and times[0] > 0 and times[1] > 0:
        slopes = []
        for i in range(1, len(times)):
            if times[i-1] > 0 and times[i] > 0:
                log_n_ratio = math.log(sizes[i] / sizes[i-1])
                log_t_ratio = math.log(times[i] / times[i-1])
                slopes.append(log_t_ratio / log_n_ratio)

        if slopes:
            avg_slope = sum(slopes) / len(slopes)
            if avg_slope < 0.2:
                complexity = "O(1)"
            elif avg_slope < 0.5:
                complexity = "O(log n)"
            elif avg_slope < 1.3:
                complexity = "O(n)"
            elif avg_slope < 1.7:
                complexity = "O(n log n)"
            elif avg_slope < 2.5:
                complexity = "O(n^2)"
            elif avg_slope < 3.5:
                complexity = "O(n^3)"
            else:
                complexity = "O(2^n) or worse"

            print(f"\nTime Complexity: {{complexity}} (slope: {{avg_slope:.2f}})")
            print("Reference: O(1)=0  O(n)=1  O(n^2)=2  O(n^3)=3")

if __name__ == "__main__":
    run_tests()
"#,
            solution_code = solution_code,
            title = problem.title,
            func_name = problem.function_name,
            test_cases = test_cases_json,
            complexity_generator = python_generator,
        )
    }

    /// Generate C test runner
    fn generate_c_test_runner(&self, problem: &Problem, solution_code: &str) -> String {
        // Analyze test cases to infer types
        let (param_types, return_type) = self.infer_c_types(problem);

        // Generate test case code
        let test_case_code = self.generate_c_test_cases(problem, &param_types, &return_type);

        format!(
            r#"#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <stdbool.h>
#include <limits.h>

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

// ============== TEST RUNNER ==============
int main() {{
    printf("\n==================================================\n");
    printf("Running tests for: {title}\n");
    printf("==================================================\n\n");

    int passed = 0;
    int total = 0;

{test_case_code}

    printf("\n==================================================\n");
    if (passed == total) {{
        printf("All %d tests passed!\n", total);
    }} else {{
        printf("%d/%d tests passed.\n", passed, total);
    }}
    printf("==================================================\n");

    return passed == total ? 0 : 1;
}}
"#,
            solution_code = solution_code,
            title = problem.title,
            test_case_code = test_case_code,
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
    fn generate_c_test_cases(&self, problem: &Problem, param_types: &[String], return_type: &str) -> String {
        let mut code = String::new();
        let func_name = &problem.function_name;

        for (i, tc) in problem.test_cases.iter().enumerate() {
            code.push_str(&format!("    // Test case {}\n", i + 1));
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

            // Time the execution
            code.push_str("        clock_t start = clock();\n");

            // Call function based on return type
            if return_type == "int*" {
                code.push_str("        int returnSize;\n");
                code.push_str(&format!("        int* result = {}({}, &returnSize);\n", func_name, arg_exprs.join(", ")));
            } else {
                code.push_str(&format!("        {} result = {}({});\n", return_type, func_name, arg_exprs.join(", ")));
            }

            code.push_str("        clock_t end = clock();\n");
            code.push_str("        double duration = ((double)(end - start)) / CLOCKS_PER_SEC * 1000000;\n");

            // Compare results
            if return_type == "int*" {
                code.push_str("        int pass = arrays_equal(result, returnSize, expected, expected_size);\n");
            } else if return_type == "bool" {
                code.push_str("        int pass = (result == expected);\n");
            } else {
                code.push_str("        int pass = (result == expected);\n");
            }

            code.push_str("        if (pass) {\n");
            code.push_str(&format!("            printf(\"Test {}: PASSED (%.0fus)\\n\", duration);\n", i + 1));
            code.push_str("            passed++;\n");
            code.push_str("        } else {\n");
            code.push_str(&format!("            printf(\"Test {}: FAILED\\n\");\n", i + 1));

            // Print expected and got
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

            // Free allocated memory for array returns
            if return_type == "int*" {
                code.push_str("        if (result) free(result);\n");
            }

            code.push_str("    }\n\n");
        }

        code
    }

    /// Generate C++ test runner
    fn generate_cpp_test_runner(&self, problem: &Problem, solution_code: &str) -> String {
        // Analyze test cases to infer types
        let (param_types, return_type) = self.infer_cpp_types(problem);

        // Generate test case code
        let test_case_code = self.generate_cpp_test_cases(problem, &param_types, &return_type);

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

// ============== TEST RUNNER ==============
int main() {{
    cout << "\n==================================================" << endl;
    cout << "Running tests for: {title}" << endl;
    cout << "==================================================" << endl << endl;

    Solution sol;
    int passed = 0;
    int total = 0;

{test_case_code}

    cout << "\n==================================================" << endl;
    if (passed == total) {{
        cout << "All " << total << " tests passed!" << endl;
    }} else {{
        cout << passed << "/" << total << " tests passed." << endl;
    }}
    cout << "==================================================" << endl;

    return passed == total ? 0 : 1;
}}
"#,
            solution_code = solution_code,
            title = problem.title,
            test_case_code = test_case_code,
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
    fn generate_cpp_test_cases(&self, problem: &Problem, param_types: &[String], return_type: &str) -> String {
        let mut code = String::new();
        let func_name = &problem.function_name;

        // Determine if we need order-insensitive comparison
        let any_order = problem.description.to_lowercase().contains("any order");

        for (i, tc) in problem.test_cases.iter().enumerate() {
            code.push_str(&format!("    // Test case {}\n", i + 1));
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

            // Time the execution
            code.push_str("        auto start = chrono::high_resolution_clock::now();\n");
            code.push_str(&format!("        auto result = sol.{}({});\n", func_name, arg_names.join(", ")));
            code.push_str("        auto end = chrono::high_resolution_clock::now();\n");
            code.push_str("        auto duration = chrono::duration_cast<chrono::microseconds>(end - start).count();\n");

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

            code.push_str(&format!("        bool pass = {};\n", compare_expr));
            code.push_str("        if (pass) {\n");
            code.push_str(&format!("            cout << \"Test {}: PASSED (\" << duration << \"μs)\" << endl;\n", i + 1));
            code.push_str("            passed++;\n");
            code.push_str("        } else {\n");
            code.push_str(&format!("            cout << \"Test {}: FAILED\" << endl;\n", i + 1));
            code.push_str(&format!("            cout << \"  Expected: \" << {} << endl;\n", expected_to_str));
            code.push_str(&format!("            cout << \"  Got:      \" << {} << endl;\n", result_to_str));
            code.push_str("        }\n");
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
