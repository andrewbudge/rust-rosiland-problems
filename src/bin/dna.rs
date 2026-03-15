use std::collections::HashMap;

fn main() {
    let dna_seq = "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC";

    let mut nucleotide_counts = HashMap::new();

    for nucleotide in dna_seq.chars() {
        let count = nucleotide_counts.entry(nucleotide).or_insert(0);
        *count += 1;
    }

    println!(
        "{} {} {} {}",
        nucleotide_counts.get(&'A').unwrap_or(&0),
        nucleotide_counts.get(&'C').unwrap_or(&0),
        nucleotide_counts.get(&'G').unwrap_or(&0),
        nucleotide_counts.get(&'T').unwrap_or(&0)
    );
}
