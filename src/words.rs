use std::{collections::HashMap, fs::File, io::{BufRead, BufReader, Lines}};
use crate::signatures::signature;

pub fn collect_words(path: String) -> HashMap<Vec<char>, Vec<String>> {
    let r = BufReader::new(File::open(path)
        .expect("Failed to open file"));
    let lines: Lines<BufReader<File>> = r.lines();

    let mut signature_map: HashMap<Vec<char>, Vec<String>> = HashMap::new();
    for l in lines.map_while(Result::ok) {
        let sig = signature(&l);
        match signature_map.get_mut(&sig) {
            Some(wordlist) => {
                wordlist.push(l);
            },
            None => {
                signature_map.insert(sig, vec![l]);
            }
        };
    }
    return signature_map;
}

pub fn write_words(signatures: Vec<Vec<char>>, signature_map: HashMap<Vec<char>, Vec<String>>) {
    for (i, sig) in signatures.iter().enumerate() {
        println!("Level {} words:", i);
        match signature_map.get(sig) {
            Some(words) => {
                for w in words.iter() {
                    println!("{}", w);
                }
            },
            None => ()
        };
        println!();
    }
    println!("Longest chain length: {}", signatures.len());
}
