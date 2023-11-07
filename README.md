# rs_array
**Julia**-like Multi-dimensional Array for Rust

## Usage
### Initilization
```rust

    let m = matrix![1 2; 3 4; 5 6];
    {
        let m_trans = m.transpose();
        let m_reshape = reshape!(m, 1, 6);
        println!("{}", m);
        println!("{}", m_trans);
        println!("{}", m_reshape);
    }

    let m = randn!(f64, 5, 4, 3, 2);
    println!("{}", m.map(|x| x * x));
```
