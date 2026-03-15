fn main() {
    let dna_seq = "GATGGAACTTGACTACGTAAATT";

    let rna_seq = &dna_seq.replace("T", "U");

    println!("{}", rna_seq)
}
