# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`bevy_ui_text_input` is a text input plugin for Bevy UI that provides rich text editing capabilities using cosmic-text. It's a workspace member of the larger Velo project and offers features like undo/redo, text selection, IME support, clipboard operations, and various input modes.

## Common Development Commands

### Build and Run

```bash
# Run example (primary way to test the plugin)
cargo run --example text_input

# Build the library
cargo build

# Build with release optimizations
cargo build --release

# Run all examples
cargo run --example single_line
cargo run --example integer_input
cargo run --example text_input
```

### Testing

```bash
# Run tests for this crate
cargo test

# Run tests with output
cargo test -- --nocapture
```

### Linting and Formatting

```bash
# Format code (required before commits)
cargo fmt

# Run clippy linter
cargo clippy

# Check without building
cargo check
```

## Architecture Overview

### Core Components

- **TextInputNode**: Main component that creates a text input field
- **TextInputBuffer**: Manages the underlying text buffer using cosmic-text
- **TextInputStyle**: Styling configuration for text inputs
- **TextInputQueue**: Handles queued text input actions
- **TextInputLayoutInfo**: Stores layout information for rendering

### Plugin Structure

The `TextInputPlugin` integrates with Bevy's systems:
- Adds input dispatch functionality via `InputDispatchPlugin`
- Manages global state through `TextInputGlobalState` resource
- Handles rendering pipeline via `TextInputPipeline`
- Provides clipboard support through platform-specific implementations

### Key Systems

- **Text Processing**: `process_text_input_queues`, `update_text_input_contents`
- **IME Support**: `listen_ime_events`, `toggle_ime_on_focus`
- **Mouse Interaction**: `on_text_input_pressed`, `on_drag_text_input`, `mouse_wheel_scroll`
- **Keyboard Input**: `on_focused_keyboard_input`
- **Rendering**: `extract_text_input_nodes`, `extract_text_input_prompts`

### Platform-Specific Features

The crate uses conditional compilation for platform features:
- **Native (Windows/Unix)**: Uses `arboard` for clipboard operations
- **WASM**: Uses web-sys for browser clipboard API
- **Clipboard**: Abstracted through `clipboard.rs` module

### Input Modes

- **Normal**: Standard text input
- **Integer**: Validates integer input
- **Decimal**: Validates decimal number input  
- **Hexadecimal**: Validates hexadecimal input

## Key Technical Details

### Text Editing
- Uses cosmic-text's `Editor` and `Buffer` for text management
- Supports undo/redo through action history
- Implements text selection with keyboard and mouse
- Handles multi-click selection (double for word, triple for paragraph)

### IME Support
- Full IME (Input Method Editor) support for Chinese and other languages
- Proper handling of composition events
- Integration with system language preferences via `sys-locale`

### Rendering Pipeline
- Custom rendering pipeline integrated with Bevy UI
- Extracts text nodes in the extraction schedule
- Manages font atlases separately from bevy_text

## Development Guidelines

### Adding New Features

1. New input actions should be added to `actions.rs`
2. Rendering changes go in `render.rs` and `text_input_pipeline.rs`
3. Input handling logic belongs in `edit.rs`
4. Platform-specific code should be in `clipboard.rs` with proper cfg attributes

### Testing Changes

1. Create or modify examples to demonstrate new features
2. Test on both native and WASM targets when applicable
3. Verify clipboard functionality on target platforms
4. Check IME functionality with different input methods

### Common Patterns

- Use `TextInputQueue` to batch multiple text operations
- Leverage `TextInputAction` enum for all text modifications
- Maintain undo/redo history through the action system
- Use `InputFocus` resource to manage active input

## Known Limitations

- Scrolling can be glitchy if line height isn't exact divisor of input height
- Creates separate font atlases from bevy_text
- No responsive sizing support yet
- No rich text or syntax highlighting
- No world UI or Text2d support
- Android clipboard not supported