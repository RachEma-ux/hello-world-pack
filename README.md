# Hello World Pack

A simple WASM pack demonstrating the Builder app workflow.

## What it does

- Prints a greeting message
- Shows environment information
- Computes Fibonacci(20) as a demo computation
- Simulates multi-step task processing

## Installation

1. Open Builder app on Android
2. Go to Packs tab
3. Select this repository
4. Choose a release version
5. Tap Install

## Building

```bash
# Install Rust with WASM target
rustup target add wasm32-wasip1

# Build
cargo build --release --target wasm32-wasip1

# Run locally with Wasmtime
wasmtime target/wasm32-wasip1/release/main.wasm
```

## Expected Output

```
========================================
   Hello from Builder WASM Runtime!
========================================

[INFO] Pack: Hello World Pack v1.0.0
[INFO] Runtime: wasm32-wasip1

[INFO] Environment variables available: X
[INFO] Fibonacci(20) = 6765

[TASK] Processing data...
[TASK] Step 1/5 complete
[TASK] Step 2/5 complete
[TASK] Step 3/5 complete
[TASK] Step 4/5 complete
[TASK] Step 5/5 complete

[SUCCESS] Pack execution completed successfully!
========================================
```

## License

MIT
