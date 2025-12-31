use anyhow::Result;
use rust_embed::Embed;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
}
