
fn fib_recursive(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fib_recursive(n - 1) + fib_recursive(n - 2);
    }
}

// calculate the fibonacci number iteratively
fn fib_iterative(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    let mut c = 0;
    for _ in 0..n {
        c = a + b;
        a = b;
        b = c;
    }
    return a;
}

fn main() {
    println!("--- RECURSIVE ---");
    for i in 0..21 {
        println!("fib_recursive({}) = {}", i, fib_recursive(i));
    }
    println!("--- ITERATIVE ---");
    for i in 0..21 {
        println!("fib_iterative({}) = {}", i, fib_iterative(i));
    }
    println!("--- END ---");
}

