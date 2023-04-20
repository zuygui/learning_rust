
fn hanoi(n: i32, from: i32, to: i32, via: i32) {

    if n > 0 {
        hanoi(n - 1, from, via, to);
        println!("Move disk from pole {} to pole {}", from, to);
        hanoi(n - 1, via, to, from)
    }
}

fn main() {
    hanoi(4, 1, 2, 3);
}

