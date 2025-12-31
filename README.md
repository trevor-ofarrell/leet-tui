# LeetCode TUI

A terminal user interface (TUI) application for solving LeetCode problems with an embedded Neovim editor.

## Features

- **Home page with problem list**: Browse all available LeetCode problems with difficulty indicators
- **Split-screen interface**: 1/3 for problem description, 2/3 for Neovim editor
- **Automatic boilerplate generation**: Each problem gets custom boilerplate code and test cases
- **Run tests locally**: Press Ctrl+R to run your solution against test cases with instant feedback
- **Color-coded results**: Test output shows PASSED in green, FAILED in red
- **Embedded Neovim**: Full Neovim functionality within the TUI
- **JavaScript solutions**: Default language is JavaScript (other languages coming soon)
- **Organized solution files**: Solutions saved in `solutions/` directory as `problem_X.js`
- **Proper window sizing**: Neovim automatically fits the available space
- **Focus switching**: Toggle between question pane and editor pane
- **Scrollable question view**: Navigate through problem descriptions
- **Proper input forwarding**: All keyboard shortcuts work in Neovim (arrows, Ctrl combinations, etc.)

## Architecture

The application uses:
- **ratatui**: Terminal UI framework
- **portable-pty**: PTY system for spawning Neovim
- **vt100**: Terminal emulator parser
- **tui-term**: Widget for rendering terminal output
- **crossterm**: Cross-platform terminal manipulation

## Prerequisites

- Rust (1.70+)
- Neovim installed and available in PATH

## Building

```bash
cargo build --release
```

## Running

```bash
cargo run
```

## Keyboard Shortcuts

### Home Page
- **Up/Down**: Navigate problem list
- **Enter**: Select a problem
- **Ctrl+C**: Quit application

### Question View
- **Ctrl+R**: Save file and run tests (auto-saves before testing)
- **Ctrl+Q**: Switch focus between question pane and editor pane
- **Ctrl+H**: Back to home page
- **Ctrl+C**: Quit application
- **Esc**: Close test results popup
- **Up/Down**: Scroll question (when question pane is focused)
- **PageUp/PageDown**: Fast scroll question (when question pane is focused)
- **Home**: Jump to top of question (when question pane is focused)
- **All Neovim shortcuts**: Work when editor pane is focused

## Project Structure

```
src/
├── main.rs       # Main application logic and UI rendering
├── pty.rs        # PTY manager for Neovim integration
├── input.rs      # Keyboard input handling and ANSI escape sequence mapping
└── leetcode.rs   # LeetCode API integration (currently with sample data)
```

## How It Works

### PTY Integration

The application spawns Neovim in a pseudo-terminal (PTY) and renders its output in a Ratatui widget. A background thread continuously reads from the PTY and updates a VT100 parser, which maintains the terminal state.

### Input Forwarding

When the editor pane is focused, all keyboard events are converted to ANSI escape sequences and sent to the PTY. This includes:
- Regular characters
- Control combinations (Ctrl+A, Ctrl+W, etc.)
- Arrow keys and function keys
- Special keys (Home, End, PageUp, PageDown, etc.)

### Focus Management

The application maintains a focus state that determines which pane receives input:
- **Question pane**: Arrow keys scroll the problem description
- **Editor pane**: All input is forwarded to Neovim

## Extending

### Adding Real LeetCode API Integration

Currently, the app displays a sample problem. To integrate with the real LeetCode API:

1. Update `src/leetcode.rs` to make GraphQL requests to LeetCode
2. Add authentication support
3. Implement problem selection UI

### Customizing Neovim

You can customize the Neovim instance by:
- Setting environment variables (e.g., `NVIM_APPNAME` for different configs)
- Passing command-line arguments in `src/pty.rs:25`
- Creating a custom init file for the TUI context

The app currently opens `solution.js` by default. To support other languages, modify the file extension in `src/pty.rs:25`.

## Troubleshooting

### Keys not working in Neovim

If certain key combinations don't work, you may need to add them to the `key_to_bytes` function in `src/input.rs`.

### Terminal size issues

The application handles resize events and updates the PTY size accordingly. If you experience rendering issues, try resizing your terminal.

### Neovim colors look wrong

The VT100 parser supports basic colors. If you use a complex Neovim theme, some colors may not render perfectly in the embedded terminal.

## Future Enhancements

- [ ] Real LeetCode API integration
- [ ] Problem selection menu
- [ ] Multiple test case support
- [ ] Submit solution functionality
- [ ] History of attempted problems
- [ ] Custom Neovim configuration per problem type
- [ ] Split terminal for running tests

## License

MIT
