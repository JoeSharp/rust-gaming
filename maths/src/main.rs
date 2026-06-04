use maths::matrix;
use maths::matrix::Matrix;

fn main() {
    println!("Hello, world!");

    let m1 = Matrix {
        rows: 2,
        columns: 2,
        data: vec![1.0, 2.0, 3.0, 4.0],
    };

    let m2 = matrix!(
        rows: 2,
        cols: 2,
        4, 5;
        8, 9,
    );

    let m_result = m1.sum(&m2).unwrap();
    //m2.get_index_ok(5, 6);

    println!("{}", m1);
    println!("+");
    println!("{}", m2);
    println!("=");
    println!("{}", m_result);
}
