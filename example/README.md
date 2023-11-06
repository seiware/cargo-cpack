# `cargo-cpack` Sample Project

This is a sample project to demonstrate how to use `cargo-cpack` for code bundling in a Rust project. It's set up to show how `cargo-cpack` can pack your executable Rust source files from the `bin` directory, along with the associated library modules, into single, submission-ready files.

## Project Structure

Below is the project directory layout:

```txt
.
├── Cargo.toml
├── bin
│   ├── solution1.rs
│   ├── solution2.rs
│   ├── solution3.rs
│   └── solution4.rs
├── lib.rs
├── a.rs
├── b.rs
└── a
    ├── b.rs
    ├── c.rs
    ├── d.rs
    ├── e.rs
    └── c
        └── d.rs
```

`cargo-cpack` is designed to work with projects that structure their executable code within the `bin` directory and utilize shared code defined in `lib.rs` or other modules within the source tree.

## How to Run `cargo-cpack`

To bundle your code with `cargo-cpack`, navigate to the root of this sample project in your terminal and execute the following command:

```sh
cargo cpack --path ./bin/solution1.rs
```

Replace `solution1.rs` with the appropriate solution file you wish to pack. You can direct the output to a file using:

```sh
cargo cpack --path ./bin/solution1.rs > packed_solution1.rs
```

## Output

The packed source code generated by `cargo-cpack` for each solution file in the `bin` directory will include all the referenced modules and dependencies neatly consolidated into a single file. You can find the original source files for each solution within the `bin` directory.

## Using `cargo-cpack` in Your Projects

To utilize `cargo-cpack` in your own Rust projects, ensure that:

- Executable source files are placed within the `bin` directory.
- Shared code such as functions, structs, or modules, is accessible through the `lib.rs` file or other module files in the project.
- Project dependencies are correctly defined in your `Cargo.toml`.

With these steps in place, `cargo-cpack` will be able to bundle your code for a hassle-free submission process.