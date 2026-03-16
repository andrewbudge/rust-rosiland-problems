fn main() {
    let dna_seq = "AAAACCCGGT";

    let reverse_complement_dna_seq = dna_seq
        .chars()
        .map(|nucleotide| match nucleotide {
            'A' => 'T',
            'T' => 'A',
            'C' => 'G',
            'G' => 'C',
            _ => nucleotide,
        })
        .rev()
        .collect::<String>();

    println!("{}", reverse_complement_dna_seq);
}
