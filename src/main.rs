use std::env;

use words::{collect_words, write_words};
use signatures::{disjoint_signatures, find_longest_chain};

mod words;
mod signatures;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("Commencing...");
    let signature_map = collect_words(&args[1]);
    drop(args);
    println!("Successfully mapped {} signatures. Begin finding disjoint signatures...", signature_map.len());
    let disjoint_signatures = disjoint_signatures(
        signature_map
        .keys()
        .cloned()
        .collect()
    );
    println!("Successfully mapped disjoint signatures. Begin finding chains...");
    let longest_chain = find_longest_chain(disjoint_signatures);
    write_words(longest_chain, signature_map);
}
