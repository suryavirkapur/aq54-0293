use std::fs::File;
use std::io::{BufReader, BufRead, Write};


fn add_modulo(a: i32, b: i32, modulus: i32) -> i32 {
    (a + b) % modulus
}

fn right_cosets(h: &Vec<Vec<i32>>, modulus: i32) -> Vec<Vec<Vec<i32>>> {
    let mut cosets = Vec::new();

    for a in 0..modulus {
        let coset: Vec<Vec<i32>> = h.iter().map(|h_element| vec![add_modulo(a, h_element[0], modulus)]).collect();
        cosets.push(coset);
    }

    cosets
}

fn print_cosets<W: Write>(cosets: Vec<Vec<Vec<i32>>>, writer: &mut W) {
    for (i, coset) in cosets.iter().enumerate() {
        writeln!(writer, "Coset {}: {:?}", i + 1, coset).expect("Error writing to file");
    }
}

fn main() {
    // Read H from file
    let h_file = File::open("aq54ip-292").expect("Error opening file");
    let h_reader = BufReader::new(h_file);
    let h: Vec<Vec<i32>> = h_reader
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| {
            let elements: Vec<i32> = line.split_whitespace().filter_map(|s| s.parse().ok()).collect();
            if elements.len() == 1 {
                Some(vec![elements[0]])
            } else {
                None
            }
        })
        .collect();

    let modulus = 8;

    // Compute right cosets
    let cosets = right_cosets(&h, modulus);

    // Write cosets to file
    let output_file = File::create("aq54op-292").expect("Error creating file");
    let mut output_writer = std::io::BufWriter::new(output_file);
    print_cosets(cosets, &mut output_writer);
}
