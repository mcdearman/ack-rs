fn main() {
    for i in 0..10 {
        for j in 0..10 {
            println!("ack({}, {}) = {}", i, j, ack(i, j))
        }
    }
}

fn ack(m: u64, n: u64) -> u64 {
    if m == 0 {
        n + 1
    } else if n == 0 {
        ack(m - 1, 1)
    } else {
        ack(m - 1, ack(m, n - 1))
    }
}
