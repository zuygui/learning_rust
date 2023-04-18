# Fibonnaci suite

## Logic of the exercice

```
U0 = 1
U1 = 2

Un = Un-1 + Un-2
```

## TODO

write a program with recursivity and not recursivity in Rust from `U0` to `U20` with cache management for kill the risk of CPU Crash.

## Utils resources

[Wikipedia FR](https://fr.wikipedia.org/wiki/Suite_de_Fibonacci)

## Output of the program

```
--- RECURSIVE ---
fib_recursive(0) = 0
fib_recursive(1) = 1
fib_recursive(2) = 1
fib_recursive(3) = 2
fib_recursive(4) = 3
fib_recursive(5) = 5
fib_recursive(6) = 8
fib_recursive(7) = 13
fib_recursive(8) = 21
fib_recursive(9) = 34
fib_recursive(10) = 55
fib_recursive(11) = 89
fib_recursive(12) = 144
fib_recursive(13) = 233
fib_recursive(14) = 377
fib_recursive(15) = 610
fib_recursive(16) = 987
fib_recursive(17) = 1597
fib_recursive(18) = 2584
fib_recursive(19) = 4181
fib_recursive(20) = 6765
--- ITERATIVE ---
--- END ---

 ~/Documents/learning_rust   LEVEL-1-Fibonacci-suite !1 ?5                                                               1.71.0-nightly  22:49:14 
❯ cargo run
   Compiling fibonnaci-suite v0.1.0 (/home/zuygui/Documents/learning_rust)
warning: value assigned to `c` is never read
  --> src/main.rs:16:13
   |
16 |     let mut c = 0;
   |             ^
   |
   = help: maybe it is overwritten before being read?
   = note: `#[warn(unused_assignments)]` on by default

warning: `fibonnaci-suite` (bin "fibonnaci-suite") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/fibonnaci-suite`
--- RECURSIVE ---
fib_recursive(0) = 0
fib_recursive(1) = 1
fib_recursive(2) = 1
fib_recursive(3) = 2
fib_recursive(4) = 3
fib_recursive(5) = 5
fib_recursive(6) = 8
fib_recursive(7) = 13
fib_recursive(8) = 21
fib_recursive(9) = 34
fib_recursive(10) = 55
fib_recursive(11) = 89
fib_recursive(12) = 144
fib_recursive(13) = 233
fib_recursive(14) = 377
fib_recursive(15) = 610
fib_recursive(16) = 987
fib_recursive(17) = 1597
fib_recursive(18) = 2584
fib_recursive(19) = 4181
fib_recursive(20) = 6765
--- ITERATIVE ---
fib_iterative(0) = 0
fib_iterative(1) = 1
fib_iterative(2) = 1
fib_iterative(3) = 2
fib_iterative(4) = 3
fib_iterative(5) = 5
fib_iterative(6) = 8
fib_iterative(7) = 13
fib_iterative(8) = 21
fib_iterative(9) = 34
fib_iterative(10) = 55
fib_iterative(11) = 89
fib_iterative(12) = 144
fib_iterative(13) = 233
fib_iterative(14) = 377
fib_iterative(15) = 610
fib_iterative(16) = 987
fib_iterative(17) = 1597
fib_iterative(18) = 2584
fib_iterative(19) = 4181
fib_iterative(20) = 6765
--- END ---
```
