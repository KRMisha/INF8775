use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use petgraph::matrix_graph::{UnMatrix, NodeIndex};

pub fn load_graph(filename: &Path) -> Result<UnMatrix<u8, ()>, Box<dyn Error>> {
    let buffered = BufReader::new(File::open(filename)?);
    let mut lines_it = buffered.lines().map(|l| l.unwrap());

    // Read adjacency matrix size
    let first_line = lines_it.next().unwrap();
    let matrix_size = first_line.trim().parse()?;

    // Initialize graph
    let mut graph = UnMatrix::with_capacity(matrix_size);

    // Create nodes
    for _ in 0..matrix_size {
        graph.add_node(0u8); // TODO: Check if storing a weight in the node is still needed
    }

    // Create edges
    for (i, line) in lines_it.enumerate() {
        for (j, number) in line.trim().split(char::is_whitespace).enumerate() {
            let value: u8 = number.trim().parse()?;
            if value == 1 {
                graph.update_edge(NodeIndex::new(i), NodeIndex::new(j), ());
            }
        }
    }

    Ok(graph)
}

pub fn print_result(colors: &Vec<usize>) {
    let color_count = colors.iter().max().unwrap() + 1;
    println!("{}", color_count);

    let color_str = colors.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ");
    println!("{}", color_str);
}
