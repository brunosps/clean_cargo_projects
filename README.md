# Rust Script: Recursive `cargo clean`

This Rust script recursively navigates through directories starting from a specified path (provided as a command-line argument). It checks each directory for the presence of a `Cargo.toml` file, which indicates a Rust project. If it finds one, the script executes the `cargo clean` command in that directory to clean the project build artifacts.

## Key Features
1. **Recursive Search**: The script searches through all subdirectories of the given starting path.
2. **Detection of `Cargo.toml`**: It identifies Rust project directories by the presence of a `Cargo.toml` file.
3. **Execution of `cargo clean`**: For each detected project, it runs the `cargo clean` command to remove build files.
4. **Error Handling**: 
   - Handles errors gracefully, such as missing directories, failed commands, or inaccessible files.
   - Prints meaningful error messages to assist debugging.
5. **Clean Output**:
   - Suppresses unnecessary command outputs (`stdout` and `stderr`).
   - Displays concise messages indicating success or failure for each project.

## Usage
1. **Compile the Script**:
   ```bash
   rustc clean_cargo_projects.rs

2. **Compile the Script**:
   ```bash
   ./clean_cargo_projects /path/to/start_directory

## Example
Given the directory structure:

```bash
/projects
  /project1
    Cargo.toml
  /project2
    Cargo.toml
  /other
    /project3
      Cargo.toml
```

Running:
  ```bash
    ./clean_cargo_projects /projects
  ```

Will execute cargo clean in:

- /projects/project1

- /projects/project2

- /projects/other/project3

This tool is especially useful for developers managing multiple Rust projects who need to clean them up efficiently.


