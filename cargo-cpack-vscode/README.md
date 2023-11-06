# Cargo CPack Clipboard Copier

This extension allows you to run `cargo-cpack` with the path of the currently open file in Visual Studio Code, and then copies the command's output directly to your clipboard.

## Features

- Run `cargo-cpack` using the file path of the currently active editor window in VS Code.
- Automatically copy the output of the `cargo-cpack` command to the system's clipboard.

## Requirements

Before you use this extension, you must have the following installed:

- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) (Rust's package manager)
- The `cargo-cpack` command must be available in your system's PATH.

## Extension Settings

This extension contributes the following commands:

- `extension.cargoCpackToClipboard`: Runs `cargo-cpack` with the current file and copies output to clipboard.

## Usage

1. Open the file you want to process with `cargo-cpack`.
2. Execute the command `Cargo CPack: Copy Output to Clipboard` through the Command Palette (`Ctrl+Shift+P` on Windows/Linux or `Cmd+Shift+P` on macOS).
3. The result from `cargo-cpack` will be copied to your clipboard.

## Known Issues

- File paths with special characters may not be processed correctly and require additional handling.
- This extension assumes that the `cargo-cpack` command can run without additional arguments or interactive input.
