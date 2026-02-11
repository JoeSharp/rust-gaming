use graphics::matrix::Matrix;

fn main() {
    println!("Hello, world!");

    let m = Matrix {
        rows: 2,
        columns: 2,
        data: vec![1.0, 2.0, 3.0, 4.0],
    };

    println!("{:?}", m);
}
