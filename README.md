# cargo-cpack

`cargo-cpack` is a Rust utility tool designed to help competitive programmers pack their Rust source code from the `bin` directory into a single file. This tool streamlines the process of preparing code for submission to competitive programming judge systems.

## Features

- Bundles multiple source files into a single file for easier submission.
- Handles project dependencies and modules intelligently.
- Optional code formatting using `rustfmt`.
- Ability to output only the generated code.

## Installation

To install `cargo-cpack`, you can use `cargo`'s install command. Ensure you have Cargo installed and run the following command in your terminal:

```sh
git clone https://github.com/seiware/cargo-cpack.git
cd cargo-cpack
cargo install --path .
```

## Usage

To use `cargo-cpack`, navigate to your Rust project directory and run:

```sh
cargo-cpack <path-to-your-bin-directory>
```

### Command Line Arguments

- `path`: The path to the source to be packed.
- `--format` (optional): Formats the output code if set to `true`.
- `--gen_code_only` (optional): Generates only the code required for the modules identified to be used in `bin` directory.

Example usage:

```sh
cargo-cpack ./src/bin/my_solution.rs 
```

## Output

The tool will output the packed source to stdout. You can redirect this output to a file using the shell redirection operator (`>`), for example:

```sh
cargo-cpack -f ./src/bin/my_solution.rs > packed_solution.rs
```
