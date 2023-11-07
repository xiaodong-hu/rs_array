# rs_array
**Julia**-like Multi-dimensional Array for Rust

## Usage
### Initilization
```rust
    let m = matrix![1 2; 3 4; 5 6];
    println!("{m}");
    println!("{}", reshape!(m, 1, 6)); // m is consumed here
    
    let m = matrix![2 3 4; 5 6 7];
    println!("{}", m.map(|x| x * 2)); // m is NOT consumed after map
    println!("{m}");

    let m1 = randn!(f64, dim, dim);
    let m2 = randn!(f64, dim, dim);
    println!("{}", m1.mul_standard(&m2)); // O(n^3) matrix multiplication
```
