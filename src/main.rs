use crate::array_basic::Array;

mod scalar;

mod array_basic;
mod array_display;
mod matrix;
// mod scalar;

fn main() {
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
}
