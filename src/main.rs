use words::collect_words;
use signatures::{disjoint_signatures, find_longest_chain};

mod words;
mod signatures;

fn main() {
    let sigs_and_words = collect_words("test.txt".to_string());
    let disjoint_signatures = disjoint_signatures(
        sigs_and_words
            .keys()
            .cloned()
            .collect()
    );
    let longest_chain = find_longest_chain(disjoint_signatures);
    print!("{:?}", longest_chain);
}
