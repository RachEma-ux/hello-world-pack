use std::env;

fn main() {
    println!("========================================");
    println!("   Hello from Builder WASM Runtime!");
    println!("========================================");
    println!();

    // Display environment info
    println!("[INFO] Pack: Hello World Pack v1.0.0");
    println!("[INFO] Runtime: wasm32-wasip1");
    println!();

    // Check for any environment variables
    let env_count = env::vars().count();
    println!("[INFO] Environment variables available: {}", env_count);

    // List some safe env vars
    if let Ok(pwd) = env::var("PWD") {
        println!("[INFO] Working directory: {}", pwd);
    }

    // Perform a simple computation
    let result = compute_fibonacci(20);
    println!("[INFO] Fibonacci(20) = {}", result);

    // Simulate some work
    println!();
    println!("[TASK] Processing data...");
    for i in 1..=5 {
        println!("[TASK] Step {}/5 complete", i);
    }

    println!();
    println!("[SUCCESS] Pack execution completed successfully!");
    println!("========================================");
}

fn compute_fibonacci(n: u32) -> u64 {
    if n <= 1 {
        return n as u64;
    }
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}
