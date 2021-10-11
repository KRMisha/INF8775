use std::time::Instant;

use ndarray::{arr2, Array2, s, concatenate, Axis, stack};

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
            multiply_matrices_strassen(&(&matrix_1_slices[0] + &matrix_1_slices[3]), &(&matrix_2_slices[0] + &matrix_2_slices[3])), // M1 = (A1,1 + A2,2) * (B1,1 + B2,2)
            multiply_matrices_strassen(&(&matrix_1_slices[2] + &matrix_1_slices[3]), &matrix_2_slices[0].to_owned()), // M2 = (A2,1 + A2,2) * B1,1
            multiply_matrices_strassen(&matrix_1_slices[0].to_owned(), &(&matrix_2_slices[1] - &matrix_2_slices[3])), // M3 = A1,1 * (B1,2 - B2,2)
            multiply_matrices_strassen(&matrix_1_slices[3].to_owned(), &(&matrix_2_slices[2] - &matrix_2_slices[0])), // M4 = A2,2 * (B2,1 - B1,1)
            multiply_matrices_strassen(&(&matrix_1_slices[0] + &matrix_1_slices[1]), &matrix_2_slices[3].to_owned()), // M5 = (A1,1 + A1,2) * B2,2
            multiply_matrices_strassen(&(&matrix_1_slices[2] - &matrix_1_slices[0]), &(&matrix_2_slices[0] + &matrix_2_slices[1])), // M6 = (A2,1 - A1,1) * (B1,1 + B1,2)
            multiply_matrices_strassen(&(&matrix_1_slices[1] - &matrix_1_slices[3]), &(&matrix_2_slices[2] + &matrix_2_slices[3])), // M7 = (A1,2 - A2,2) * (B2,1 + B2,2)
        ];
        
        let result_quadrants = [ // C
            &intermediate_matrices[0] + &intermediate_matrices[3] - &intermediate_matrices[4] + &intermediate_matrices[6], // C1,1 = M1 + M4 - M5 + M7
            &intermediate_matrices[2] + &intermediate_matrices[4], // C1,2 = M3 + M5
            &intermediate_matrices[1] + &intermediate_matrices[3], // C2,1 = M2 + M4
            &intermediate_matrices[0] - &intermediate_matrices[1] + &intermediate_matrices[2] + &intermediate_matrices[5], // C2,2 = M1 - M2 + M3 + M6
        ];
        let con = concatenate![Axis(1), result_quadrants[0], result_quadrants[1]];
        let sta = stack![Axis(1), result_quadrants[0], result_quadrants[1]];
        println!("result_quadrants[0]: {}", result_quadrants[0]);
        println!("result_quadrants[1]: {}", result_quadrants[1]);
        println!("con: {}", con);
        println!("sta: {}", sta);
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
