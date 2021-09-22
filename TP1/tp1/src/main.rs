use std::time::Instant;

use ndarray::arr2;

fn main() {
    println!("Hello, world!");

    // Vanilla shit
    let matrix1: [[u8; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let matrix2: [[u8; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

    let mut now = Instant::now();

    // ijk
    let mut result = [[0u8; 3]; 3];

    let n = matrix1.len();
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                result[i][j] += matrix1[i][k] * matrix2[k][j];
            }
        }
    }

    println!("{}", arr2(&result));
    println!("Time: {}ns", now.elapsed().as_nanos());

    now = Instant::now();
    // Use ndarray
    let nd_matrix1 = arr2(&matrix1);
    let nd_matrix2 = arr2(&matrix2);

    let nd_product = nd_matrix1.dot(&nd_matrix2);

    println!("{}", nd_product);
    println!("Time: {}ns", now.elapsed().as_nanos());
}
