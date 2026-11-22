# mgrep

A small grep clone written in Rust. It searches a file for lines that contain a given text.

## Usage

```bash
cargo run -- <query> <file>
```

Example:

```bash
cargo run -- rust sample.txt
```

Case-insensitive search (set the environment variable):

```bash
IGNORE_CASE=1 cargo run -- hello poem.txt
```

## Test

```bash
cargo test
```

## What I practiced
- Splitting code into `main.rs` and `lib.rs`
- `Result`, `Box<dyn Error>` and the `?` operator
- Lifetimes on `search`
- Unit tests for case-sensitive and case-insensitive search