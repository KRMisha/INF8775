use std::error::Error;
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::time::Instant;
use std::fs::File;

use ndarray::{arr2, concatenate, s, Array2, Axis};
use structopt::StructOpt;

mod cli_args;
use cli_args::{Cli, Algorithm};

const THRESHOLD: usize = 4;

fn main() {
    // Parse args
    let args = Cli::from_args();

    // Load matrices
    let matrix_1 = load_matrix(&args.matrix1_filename).expect("Error parsing matrix 1 from file");
    let matrix_2 = load_matrix(&args.matrix2_filename).expect("Error parsing matrix 2 from file");

    assert_eq!(matrix_1.shape(), matrix_2.shape(), "Incompatible shapes between matrices.");

    // Start clock
    let now = Instant::now();

    // Execute selected algorithm
    let result = match args.algorithm {
        Algorithm::Conventional => multiply_matrices_conventional(&matrix_1, &matrix_2),
        Algorithm::Strassen => multiply_matrices_strassen(&matrix_1, &matrix_2),
        Algorithm::StrassenThreshold => multiply_matrices_strassen_threshold(&matrix_1, &matrix_2, THRESHOLD),
    };

    // Calculate elapsed time
    let elapsed_ms = now.elapsed().as_secs_f64() * 1000.0;

    if args.show_result {
        print_matrix(&result)
    }

    if args.show_exec_time {
        println!("{}", elapsed_ms);
    }
}

fn load_matrix(filename: &Path) -> Result<Array2<i32>, Box<dyn Error>> {
    let buffered = BufReader::new(File::open(filename)?);
    let mut lines_it = buffered.lines().map(|l| l.unwrap());

    // Read matrix size
    let first_line = lines_it.next().unwrap();
    let matrix_size = usize::pow(2, first_line.trim().parse()?);

    // Read matrix
    let mut matrix: Array2<i32> = Array2::zeros((matrix_size, matrix_size));
    for (i, line) in lines_it.enumerate() {
        for (j, number) in line.split(char::is_whitespace).enumerate() {
            matrix[[i, j]] = number.trim().parse()?;
        }
    }

    Ok(matrix)
}

fn multiply_matrices_conventional(matrix_1: &Array2<i32>, matrix_2: &Array2<i32>) -> Array2<i32> {
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

fn multiply_matrices_strassen(matrix_1: &Array2<i32>, matrix_2: &Array2<i32>) -> Array2<i32> {
    multiply_matrices_strassen_threshold(matrix_1, matrix_2, 0)
}

fn multiply_matrices_strassen_threshold(matrix_1: &Array2<i32>, matrix_2: &Array2<i32>, threshold: usize) -> Array2<i32> {
    let n = matrix_1.shape()[0];

    if n == 1 {
        return arr2(&[[matrix_1[[0, 0]] * matrix_2[[0, 0]]]]);
    }

    if n <= threshold {
        return multiply_matrices_conventional(matrix_1, matrix_2);
    }

    let matrix_1_slices = [
        matrix_1.slice(s![0..n / 2, 0..n / 2]), // A1,1
        matrix_1.slice(s![0..n / 2, n / 2..n]), // A1,2
        matrix_1.slice(s![n / 2..n, 0..n / 2]), // A2,1
        matrix_1.slice(s![n / 2..n, n / 2..n]), // A2,2
    ];
    let matrix_2_slices = [
        matrix_2.slice(s![0..n / 2, 0..n / 2]), // B1,1
        matrix_2.slice(s![0..n / 2, n / 2..n]), // B1,2
        matrix_2.slice(s![n / 2..n, 0..n / 2]), // B2,1
        matrix_2.slice(s![n / 2..n, n / 2..n]), // B2,2
    ];

    // M
    let intermediate_matrices = [
        multiply_matrices_strassen(&(&matrix_1_slices[0] + &matrix_1_slices[3]), &(&matrix_2_slices[0] + &matrix_2_slices[3])), // M1 = (A1,1 + A2,2) * (B1,1 + B2,2)
        multiply_matrices_strassen(&(&matrix_1_slices[2] + &matrix_1_slices[3]), &matrix_2_slices[0].to_owned()),               // M2 = (A2,1 + A2,2) * B1,1
        multiply_matrices_strassen(&matrix_1_slices[0].to_owned(), &(&matrix_2_slices[1] - &matrix_2_slices[3])),               // M3 = A1,1 * (B1,2 - B2,2)
        multiply_matrices_strassen(&matrix_1_slices[3].to_owned(), &(&matrix_2_slices[2] - &matrix_2_slices[0])),               // M4 = A2,2 * (B2,1 - B1,1)
        multiply_matrices_strassen(&(&matrix_1_slices[0] + &matrix_1_slices[1]), &matrix_2_slices[3].to_owned()),               // M5 = (A1,1 + A1,2) * B2,2
        multiply_matrices_strassen(&(&matrix_1_slices[2] - &matrix_1_slices[0]), &(&matrix_2_slices[0] + &matrix_2_slices[1])), // M6 = (A2,1 - A1,1) * (B1,1 + B1,2)
        multiply_matrices_strassen(&(&matrix_1_slices[1] - &matrix_1_slices[3]), &(&matrix_2_slices[2] + &matrix_2_slices[3])), // M7 = (A1,2 - A2,2) * (B2,1 + B2,2)
    ];

    // C
    let result_quadrants = [
        &intermediate_matrices[0] + &intermediate_matrices[3] - &intermediate_matrices[4] + &intermediate_matrices[6], // C1,1 = M1 + M4 - M5 + M7
        &intermediate_matrices[2] + &intermediate_matrices[4],                                                         // C1,2 = M3 + M5
        &intermediate_matrices[1] + &intermediate_matrices[3],                                                         // C2,1 = M2 + M4
        &intermediate_matrices[0] - &intermediate_matrices[1] + &intermediate_matrices[2] + &intermediate_matrices[5], // C2,2 = M1 - M2 + M3 + M6
    ];

    let concatenated_result = concatenate![
        Axis(0),
        concatenate![Axis(1), result_quadrants[0], result_quadrants[1]],
        concatenate![Axis(1), result_quadrants[2], result_quadrants[3]]
    ];

    concatenated_result
}

fn print_matrix(matrix: &Array2<i32>) {
    let n = matrix.shape()[0];
    for i in 0..n {
        let row_str = matrix.row(i)
            .map(|x| x.to_string())
            .to_vec()
            .join(" ");
        println!("{}", row_str)
    }
}