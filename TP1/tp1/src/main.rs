use std::time::Instant;

use ndarray::{arr2, Array2, s};

fn multiply_matrices_conventional(matrix_1: &Array2<u32>, matrix_2: &Array2<u32>) -> Array2<u32> {
    let n = matrix_1.shape()[0];
    let mut result = Array2::zeros((n, n));

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                result[[i, j]] += matrix_1[[i, k]] * matrix_2[[k, j]];
            }
        }
    }

    result
}

fn multiply_matrices_strassen(matrix_1: &Array2<u32>, matrix_2: &Array2<u32>) -> Array2<u32> {
    let n = matrix_1.shape()[0];
    let mut result = Array2::zeros((n, n));

    if n == 1 {
        result[[0, 0]] = matrix_1[[0, 0]] * matrix_2[[0, 0]];
    } else {
        let matrix_1_slices = [
            matrix_1.slice(s![0..n/2, 0..n/2]), // A1,1
            matrix_1.slice(s![0..n/2, n/2..n]), // A1,2
            matrix_1.slice(s![n/2..n, 0..n/2]), // A2,1
            matrix_1.slice(s![n/2..n, n/2..n]), // A2,2
        ];
        let matrix_2_slices = [
            matrix_2.slice(s![0..n/2, 0..n/2]), // B1,1
            matrix_2.slice(s![0..n/2, n/2..n]), // B1,2
            matrix_2.slice(s![n/2..n, 0..n/2]), // B2,1
            matrix_2.slice(s![n/2..n, n/2..n]), // B2,2
        ];

        let intermediate_matrices = [ // M
            multiply_matrices_strassen(&(&matrix_1_slices[0] + &matrix_1_slices[3]), &(&matrix_2_slices[0] + &matrix_2_slices[3])),
            // TODO
        ];
        println!("{}", intermediate_matrices[0]);
    }

    result
}

fn main() {
    let matrix_1 = arr2(&[[1, 2, 3, 4], [4, 5, 6, 7], [7, 8, 9, 10], [10, 11, 12, 14]]);
    let matrix_2 = arr2(&[[1, 2, 3, 4], [4, 5, 6, 7], [7, 8, 9, 10], [10, 11, 12, 14]]);

    // Conventional
    let now = Instant::now();

    let result = multiply_matrices_conventional(&matrix_1, &matrix_2);

    println!("{}", &result);
    println!("Time: {}ns", now.elapsed().as_nanos());

    // Strassen
    multiply_matrices_strassen(&matrix_1, &matrix_2);
}
