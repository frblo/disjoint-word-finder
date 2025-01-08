use std::collections::HashMap;

pub fn signature(word: String) -> Vec<char> {
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

pub fn collect_words(path: String) -> HashMap<Vec<char>, Vec<String>> {
    HashMap::new()
}
