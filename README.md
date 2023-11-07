# rs_array
**Julia**-like Multi-dimensional Array for Rust, so is set to be **column-major**.

## Usage
Every interface is trying to imitate the behavior of julia standard array.

### Initialization
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

### Display of Multidimensional Array
The display of julia's array is imitated, so that the float number is properly truncated and aligned. For example, for matrix:
```rust
let m = randn!(f32, 2,3);
println!("{m}")
```
gives
```
Array<f64, [2, 3]>:
  0.208576  0.341814  0.603347
  0.896649  0.008211  0.647971
```

For multi-dimensional array, the display strategy is to sliced out 2D matrices for higher dimensions as julia's multidimensional array does. For example,
```rust
let m = randn!(f64,4,3,2);
println!("{m}")
```
gives
```
Array<f64, [4, 3, 2]>:
[:, :, 0] = 
    0.2637     0.8062     0.5298 
    0.2719     0.7583     0.5620 
    0.8732     0.9599     0.1974 
    0.8368     0.6052     0.2179 

[:, :, 1] = 
    0.1466     0.8651     0.7677 
    0.6837     0.7497     0.6602 
    0.8462     0.4267     0.5393 
    0.3077     0.2844     0.5316 
```
