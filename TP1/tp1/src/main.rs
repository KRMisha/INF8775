use std::time::Instant;

use ndarray::{arr2, Array2};

fn multiply_matrices_conventional(matrix1: &Array2<u8>, matrix2: &Array2<u8>) -> Array2<u8> {
    let n = matrix1.shape()[0];
    let mut result = Array2::zeros((n, n));

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                result[[i, j]] += matrix1[[i, k]] * matrix2[[k, j]];
            }
        }
    }

    result
}

fn main() {
    let matrix1: [[u8; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let matrix2: [[u8; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

    // Conventional
    let now = Instant::now();

    let result = multiply_matrices_conventional(&arr2(&matrix1), &arr2(&matrix2));

    println!("{}", &result);
    println!("Time: {}ns", now.elapsed().as_nanos());
}
