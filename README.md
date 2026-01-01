# LeetCode TUI

A terminal user interface (TUI) application for solving LeetCode problems with an embedded Neovim editor.

## Features

- **149 NeetCode problems**: Curated problems from the NeetCode 150 list with full test suites
- **Multi-language support**: JavaScript, Python, C, and C++
- **Two test modes**:
  - **Run (Ctrl+R)**: Quick validation with 3-5 test cases
  - **Submit (Ctrl+S)**: Full validation with 50-200 test cases
- **Complexity analysis**: Automatic time/space complexity estimation after passing tests
- **Split-screen interface**: 1/3 for problem description, 2/3 for Neovim editor
- **Embedded Neovim**: Full Neovim functionality within the TUI
- **Scrollable results modal**: Navigate through test output with keyboard
- **Color-coded results**: PASSED in green, FAILED in red
- **Organized solution files**: Solutions saved to `~/.local/share/leet-tui/solutions/`

## Test Output

### Run Mode (Ctrl+R)
Verbose output showing each test case with input/output:
```
Test 1: PASSED [0.05 ms]
  Input:    [2,7,11,15], 9
  Output:   [0,1]

Test 2: PASSED [0.03 ms]
  Input:    [3,2,4], 6
  Output:   [1,2]
```

### Submit Mode (Ctrl+S)
Compact table format for large test suites:
```
+-------+--------+--------------+
| Test  | Status |     Time     |
+-------+--------+--------------+
|   1   |   ✓    |     0.05 ms  |
|   2   |   ✓    |     0.03 ms  |
|   3   |   ✗    |     0.02 ms  |
+-------+--------+--------------+

Results: 2 passed, 1 failed of 3 tests
```

### Complexity Analysis
After all tests pass, benchmarks run with statistical analysis:
```
Warming up JIT compiler...
Running benchmarks (5 rounds each, taking median)...

+-------------+------------------+------------+--------------+
|      n      |   Median Time    |    Runs    |   Std Dev    |
+-------------+------------------+------------+--------------+
|       10000 |          0.15 ms |       6640 |         1.2% |
|      100000 |          1.52 ms |        660 |         0.8% |
|     1000000 |         15.21 ms |         66 |         2.1% |
+-------------+------------------+------------+--------------+

Time Complexity:  O(n) (slope: 1.00)
Space Complexity: O(n) estimated

Note: Std Dev < 5% indicates stable results
```

## Installation

### Via npm (recommended)

```bash
npm install -g leet-tui
```

This will automatically download the correct binary for your platform.

### From source

Prerequisites:
- Rust (1.70+)
- Neovim installed and available in PATH
- For JavaScript: Bun or Node.js
- For Python: Python 3.x
- For C/C++: GCC or Clang

```bash
cargo build --release
./target/release/leet-tui
```

## Running

```bash
leet-tui
```

## Keyboard Shortcuts

### Home Page
- **Up/Down or j/k**: Navigate problem list
- **Enter**: Select a problem
- **Ctrl+C**: Quit application

### Question View
- **Ctrl+R**: Run tests (quick, 3-5 cases)
- **Ctrl+S**: Submit tests (full, 50-200 cases)
- **Ctrl+Q**: Switch focus between question pane and editor pane
- **Ctrl+H**: Back to home page
- **Ctrl+C**: Quit application
- **Up/Down**: Scroll question (when question pane is focused)
- **PageUp/PageDown**: Fast scroll question
- **Home**: Jump to top of question
- **All Neovim shortcuts**: Work when editor pane is focused

### Results Modal
- **Esc**: Close results
- **Up/Down or j/k**: Scroll results
- **PageUp/PageDown**: Fast scroll
- **Home/End or g/G**: Jump to top/bottom

## Architecture

The application uses:
- **ratatui**: Terminal UI framework
- **portable-pty**: PTY system for spawning Neovim
- **vt100**: Terminal emulator parser
- **tui-term**: Widget for rendering terminal output
- **crossterm**: Cross-platform terminal manipulation
- **rust-embed**: Embedded problem data and test cases

## Project Structure

```
src/
├── main.rs       # Main application logic and UI rendering
├── pty.rs        # PTY manager for Neovim integration
├── input.rs      # Keyboard input handling
├── leetcode.rs   # Problem data and test runner generation
└── language.rs   # Multi-language support

problems/         # Embedded problem definitions (JSON)
testcases/        # Extended test cases for run/submit modes
```

## Benchmarking Methodology

The complexity analysis uses robust benchmarking practices:

1. **Aggressive warmup**: 500 iterations across multiple input sizes to stabilize JIT
2. **Multiple rounds**: 5 benchmark rounds per size
3. **Median selection**: Uses median instead of mean to filter outliers
4. **Statistical reporting**: Shows relative standard deviation for confidence
5. **GC management**: Forces garbage collection between rounds

This produces repeatable results - look for Std Dev < 5% to confirm stability.

## Supported Languages

| Language | Runtime | File Extension |
|----------|---------|----------------|
| JavaScript | Bun (preferred) or Node.js | `.js` |
| Python | Python 3.x | `.py` |
| C | GCC/Clang | `.c` |
| C++ | G++/Clang++ | `.cpp` |

## Troubleshooting

### Keys not working in Neovim
If certain key combinations don't work, you may need to add them to the `key_to_bytes` function in `src/input.rs`.

### Terminal size issues
The application handles resize events and updates the PTY size accordingly. If you experience rendering issues, try resizing your terminal.

### Inconsistent benchmark results
If Std Dev is high (>10%), try:
- Closing other applications
- Running on a less loaded system
- The median should still be reasonably stable

### Test runner errors
Ensure the appropriate runtime is installed:
- JavaScript: `bun --version` or `node --version`
- Python: `python3 --version`
- C/C++: `gcc --version` or `clang --version`

## License

MIT
