# pinch

## Usage
```rust
fn main() {
    Pinch::from_file("example_apps/complex/inapinch.toml").build_with_defaults();
}
```

## Running Unit Tests
NOTE: Run from root directory

``bash
cargo fmt && cargo clippy --workspace --all-targets -- && cargo test --
``
