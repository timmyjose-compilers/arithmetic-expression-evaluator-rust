# arithmetic-expression-evaluator-rust

A simple interpreter for arithmetic expressions expressed as strings, in Rust. It features a REPL where you can enter arithmetic expressions 
which are then evaluated to their final form.

# Build

To build:

```
$ cargo build --release
```

To run the REPL:

```
$ cargo run
```

To run the test cases:

```
$ cargo test -- --nocapture
```

# Demo

Here is an example of an interactive session:

```
$ cargo build --release && cargo run
   Compiling arithmetic-expression-evaluator-rust v0.1.0 (/Users/z0ltan/dev/projects/compilers/arithmetic-expression-evaluator-rust)
    Finished release [optimized] target(s) in 1.05s
   Compiling arithmetic-expression-evaluator-rust v0.1.0 (/Users/z0ltan/dev/projects/compilers/arithmetic-expression-evaluator-rust)
    Finished dev [unoptimized + debuginfo] target(s) in 0.86s
     Running `target/debug/arithmetic-expression-evaluator-rust`Welcome, z0ltan. Enter arithmetic expressions to evaluate them, or Ctrl+C to quit
>> 1
1
>> 1 + 2
3
>> 2 * 3 + 1
7
>> 1 + 2 + 3
6
>> (1 + 2) + 3
6
>> (1 + (2 + 3))
6
>> (1 + 2) * 3
9
>> ((11 + 2) - 3) * (24 / 4) / 2
30
>> 100 / 10 / 2
20
>> (100 / 10) / 2
5
>> 1 +
thread 'main' panicked at '`parse_factor`: unexpected token of kind Eof.', src/eval/parser.rs:99:18
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

$ cargo build --release && cargo run
    Finished release [optimized] target(s) in 0.00s
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/arithmetic-expression-evaluator-rust`
Welcome, z0ltan. Enter arithmetic expressions to evaluate them, or Ctrl+C to quit
>> 1 + )
thread 'main' panicked at '`parse_factor`: unexpected token of kind RParen.', src/eval/parser.rs:99:18
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

$ cargo build --release && cargo run
    Finished release [optimized] target(s) in 0.00s
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/arithmetic-expression-evaluator-rust`
Welcome, z0ltan. Enter arithmetic expressions to evaluate them, or Ctrl+C to quit
>> ((11 + 2
thread 'main' panicked at 'expected to accept token of kind RParen, got token of kind Eof', src/eval/parser.rs:26:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

```

Note:

Error-handling is not the best, and the error messages are terrible, and the operators are right-associative. These might possibly be fixed in future versions.

# LICENSE

See [LICENSE.md](LICENSE.md).