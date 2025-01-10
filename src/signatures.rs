use std::{collections::HashMap, sync::mpsc, thread};

pub fn signature(word: &String) -> Vec<char> {
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

pub fn joint_signature(mut sig1: Vec<char>, sig2: Vec<char>) -> Vec<char> {
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

fn chain_builder(sig: Vec<char>, disjoint_signatures_list: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut longest_chain: Vec<Vec<char>> = Vec::new();

    for sig2 in disjoint_signatures_list.iter() {
        let disjoint = check_disjoint(&sig, sig2);
        if !disjoint {
            continue;
        }

        if disjoint_signatures_list.len() == 1 {
            return vec![sig2.clone()];
        }
        let joint_sig = joint_signature(sig.clone(), sig2.clone());
        let mut later_links = chain_builder(joint_sig, &disjoint_signatures_list[1..].to_vec());
        later_links.push(sig2.clone());

        if later_links.len() > longest_chain.len() {
            longest_chain = later_links;
        }
    }

    return longest_chain;
}

pub fn find_longest_chain(disjoint_signatures: HashMap<Vec<char>, Vec<Vec<char>>>) -> Vec<Vec<char>> {
    let (tx, rx) = mpsc::channel();
    let static_box: &'static HashMap<Vec<char>, Vec<Vec<char>>> =
        Box::leak(Box::new(disjoint_signatures));

    for (sig, disjoint_sigs) in static_box.iter() {
        println!("{:?}", sig);
        let tx_copy = tx.clone();
        thread::spawn(move || {
            let mut chain = chain_builder(sig.clone(), disjoint_sigs);
            chain.push(sig.clone());
            tx_copy.send(chain).unwrap();
        });
    }

    drop(tx);

    let mut longest_chain: Vec<Vec<char>> = Vec::new();
    for received in rx {
        if received.len() > longest_chain.len() {
            longest_chain = received;
        }
    }
    println!("whaqaa");

    return longest_chain;
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

    #[test]
    fn joint_signature_joins_not_disjoint() {
        let sig1 = signature(&"knarr".to_string());
        let sig2 = signature(&"korg".to_string());

        let res = joint_signature(sig1, sig2);
        let correct = signature(&"agknor".to_string());

        assert!(res == correct);
    }

    #[test]
    fn joint_signature_joins_disjoint() {
        let sig1 = signature(&"äpple".to_string());
        let sig2 = signature(&"korg".to_string());

        let res = joint_signature(sig1, sig2);
        let correct = signature(&"äpplekorg".to_string());

        assert!(res == correct);
    }

    #[test]
    fn chain_builder_builds_chain() {
        let sig = signature(&"apa".to_string());
        let disjoint_signatures_list = vec![
            signature(&"äpple".to_string()),
            signature(&"ekollon".to_string()),
            signature(&"ko".to_string()),
            signature(&"öl".to_string()),
            signature(&"ö".to_string()),
            signature(&"ål".to_string()),
            signature(&"kaka".to_string()),
        ];

        let res = chain_builder(sig, &disjoint_signatures_list);
        println!("res: {:?}", res);
        assert!(res.len() == 3);
    }
}
