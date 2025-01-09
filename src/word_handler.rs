use std::{collections::HashMap, fs::File, io::{BufRead, BufReader, Lines}};

fn signature(word: &String) -> Vec<char> {
    let mut word_signature: Vec<char> = vec![word.chars().nth(0).unwrap()];
    for c in word.chars() {
        for (i, tmp) in word_signature.iter().enumerate() {
            if c == *tmp {
                break;
            }
            if c < *tmp {
                word_signature.insert(i, c);
                break;
            }
            if i == word_signature.len() - 1 {
                word_signature.push(c);
                break;
            }
        }
    }

    return word_signature;
}

fn joint_signature(mut sig1: Vec<char>, sig2: Vec<char>) -> Vec<char> {
    for c2 in sig2.iter() {
        for (i, c1) in sig1.iter().enumerate() {
            if c2 == c1 {
                break;
            }
            if c2 < c1 {
                sig1.insert(i, *c2);
                break;
            }
            if i == sig1.len() - 1 {
                sig1.push(*c2);
                break;
            }
        }
    }

    return sig1;
}

fn check_disjoint(sig1: &Vec<char>, sig2: &Vec<char>) -> bool {
    for c1 in sig1.iter() {
        for c2 in sig2.iter() {
            if c1 == c2 {
                return false;
            }
            if c1 < c2 {
                break;
            }
        }
    }
    return true;
}

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

fn add_disjoint_signature(disjoint_signatures: &mut HashMap<Vec<char>, Vec<Vec<char>>>, sig1: &Vec<char>, sig2: &Vec<char>) {
    let disjoint_to_sig = match disjoint_signatures.get_mut(sig1) {
        Some(x) => x,
        None => panic!("Not good")
    };
    disjoint_to_sig.push(sig2.clone());
}

pub fn disjoint_signatures(signatures: Vec<Vec<char>>) -> HashMap<Vec<char>, Vec<Vec<char>>> {
    let mut tracker = vec![vec![false; signatures.len()]; signatures.len()];
    let mut disjoint_signatures: HashMap<Vec<char>, Vec<Vec<char>>> = HashMap::with_capacity(signatures.len());

    for sig in signatures.iter() {
        disjoint_signatures.insert(sig.clone(), Vec::new());
    }

    for (i, sig) in signatures.iter().enumerate() {
        for j in i..tracker.len() {
            if tracker[i][j] {
                continue;
            }

            tracker[i][j] = true;
            tracker[j][i] = true;

            let sig2 = &signatures[j];
            let disjoint = check_disjoint(sig, sig2);
            if !disjoint {
                continue;
            }

            add_disjoint_signature(&mut disjoint_signatures, sig, sig2);
            add_disjoint_signature(&mut disjoint_signatures, sig2, sig);
        }
    }

    return disjoint_signatures;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_disjoint_disjoint() {
        let sig1 = signature(&"apa".to_string());
        let sig2 = signature(&"moder".to_string());

        assert!(check_disjoint(&sig1, &sig2));
    }

    #[test]
    fn check_disjoint_not_disjoint1() {
        let sig1 = signature(&"apa".to_string());
        let sig2 = signature(&"pappa".to_string());

        assert!(!check_disjoint(&sig1, &sig2));
    }

    #[test]
    fn check_disjoint_not_disjoint2() {
        let sig1 = signature(&"knarr".to_string());
        let sig2 = signature(&"korg".to_string());

        assert!(!check_disjoint(&sig1, &sig2));
    }
}
