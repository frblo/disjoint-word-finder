use word_handler::{collect_words, disjoint_signatures};

mod word_handler;

fn main() {
    let sigs_and_words = collect_words("test.txt".to_string());
    let sigs = sigs_and_words.keys().cloned().collect();
    let disjoint = disjoint_signatures(sigs);
    print!("{:?}", disjoint);
}
