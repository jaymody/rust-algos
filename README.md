# Algorithms in Rust
Various data structures and algorithm implementations in rust based on [Algorithms, 4th Edition from Sedgewick and Wayne](https://algs4.cs.princeton.edu/home/).

I try to keep the implementations as simple as possible for educational purposes. As such, most of these implementations are basic/naive and do not take advantage of all possible optimizations. NOTE: I cannot guarantee the correctness of the implementations outside of the unit tests. Use at your own risk.

### Usage
```rust
use rust_algos::sorting::quick_sort;

fn main() {
    let mut arr = [1, 8, 2, 6, 9, 5, 6, 3, 4];
    quick_sort(&mut arr);
    println!("{:?}", arr);
}
```

Tests:
```
cargo test
```
