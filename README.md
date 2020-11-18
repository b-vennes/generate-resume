# Generate Resume

This project contains a Rust program which will parse a file and replace all instances of `{{text}}` with the html compiled contents of a `text.md` file located in the given directory.

## How to Run

``` Shell
cd resume-builder
cargo run --manifest-path ./resume-builder/Cargo.toml  ./resume.html ./sections index.html
```
