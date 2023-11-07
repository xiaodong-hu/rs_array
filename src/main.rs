use crate::array_general::Array;

mod scalar;

mod array_display;
mod array_general;
mod matrix;
// mod scalar;

fn main() {
    // use array_general::*;
    // let m = Array {
    //     data: vec![1, 2, 3, 4, 5, 6],
    //     shape: [2, 3].to_vec(),
    // };
    // println!("{}", m);

    // let m_new = m.reshape([1, 6].to_vec());
    // println!("{}", m_new);

    // let m_new = reshape!(m, 3, 2);
    // println!("{}", m_new);

    let m1 = matrix![
        1.0, 2.0, 3.0;
        2.0, -3.0, 4.0
    ];
    let m2 = matrix![1 2 3; 4 5 6];
    // println!("{}", m1 - m2);
    println!("{}", m1 * 5.0);

    let m3 = m2.map(|x| x as f64);

    println!("{}", reshape!(m3, 1, 6));

    println!("{}", randn!(f64, 20, 20))

    // let a = 2;
    // let b = 5.2 * a;
    // dbg!(b);
}
