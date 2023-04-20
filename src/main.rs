
fn hanoi_recursive(n: i32, from: i32, to: i32, via: i32) {

    if n > 0 {
        hanoi_recursive(n - 1, from, via, to);
        println!("Move disk from pole {} to pole {}", from, to);
        hanoi_recursive(n - 1, via, to, from)
    }
}

fn hanoi_iterate(n: i32, from: i32, to: i32, via: i32) {
    let mut stack = vec![(n, from, to, via)];
    while let Some((n, from, to, via)) = stack.pop() {
        if n > 0 {
            stack.push((n - 1, via, to, from));
            stack.push((0, from, to, via));
            stack.push((n - 1, from, via, to));
        } else {
            println!("Move disk from pole {} to pole {}", from, to);
        }
    }
}

fn main() {
    println!("--- USING RECURSION ---");
    hanoi_recursive(4, 1, 2, 3);
    println!("--- USING ITERATION ---");
    hanoi_iterate(4, 1, 2, 3);
    println!("--- END ---");
}

