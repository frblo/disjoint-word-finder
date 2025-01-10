use words::{collect_words, write_words};
use signatures::{disjoint_signatures, find_longest_chain};

mod words;
mod signatures;

fn main() {
    let signature_map = collect_words("test.txt".to_string());
    let disjoint_signatures = disjoint_signatures(
        signature_map
            .keys()
            .cloned()
            .collect()
    );
    let longest_chain = find_longest_chain(disjoint_signatures);
    write_words(longest_chain, signature_map);
}
