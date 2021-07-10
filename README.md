# The Rust Programming Language

This is my attempt to walk through the entire Rust Programming Language book by Klabnik and Nichols (2018). So far I've done the guessing game and the section on common programming concepts, with each chapter being in a folder and each subsection being a function in the `main.rs` for that chapter. As more chapters are added, more folders will be added.

## Project Status

I'm currently working on Chapter 4, which is the one on ownership! I expect to have this up by 07/12/2021.

## Project Layout

- hello_world: I'm not sure where this came from, but its useful for now
- guessing_game: Chapter 2's guessing game
- primitives: The entirety of Chapter 3
- ownership: The entirety of Chapter 4
- .gitignore: Taken from [GitHub's gitignore repository](https://github.com/github/gitignore/blob/master/Rust.gitignore)
- .vscode: Project configuration for VSCode
- Cargo.toml: Workspace configuration for Cargo, each subfolder has a separate `Cargo.toml` for project configuration
- rustcode.code-workspace: Workspace configuration for VSCode
