use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use ndarray::Array2;

pub fn load_matrix(filename: &Path) -> Result<Array2<u8>, Box<dyn Error>> {
    let buffered = BufReader::new(File::open(filename)?);
    let mut lines_it = buffered.lines().map(|l| l.unwrap());

    // Read matrix size
    let first_line = lines_it.next().unwrap();
    let matrix_size = first_line.trim().parse()?;

    // Read matrix
    let mut matrix: Array2<u8> = Array2::zeros((matrix_size, matrix_size));
    for (i, line) in lines_it.enumerate() {
        for (j, number) in line.trim().split(char::is_whitespace).enumerate() {
            matrix[[i, j]] = number.trim().parse()?;
        }
    }

    Ok(matrix)
}
