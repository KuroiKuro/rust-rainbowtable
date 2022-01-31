use sha2::{Sha256, Digest};

pub const HASH_DELIMITER: &str = ":";

pub struct WordHash {
    pub word: String,
    pub hash: String,
}

pub fn hash_word(word: &str) -> String {
    let hash = Sha256::new();
    hash.update(word);
    format!("{:x}", hash.finalize())
}

pub fn hash_word_vec(word_vec: Vec<String>) -> Vec<WordHash> {
    let hash_vec: Vec<WordHash> = Vec::new();
    for word in word_vec {
        let word_hash = WordHash {
            word: word,
            hash: hash_word(&word)
        };
        hash_vec.push(word_hash);
    }
    hash_vec
}

fn generate_hash_str(word_hash: WordHash) -> String {
    let hash_str = String::from(word_hash.word);
    hash_str.push_str(HASH_DELIMITER);
    hash_str.push_str(&format!("{}", word_hash.hash));
    hash_str
}

pub fn generate_serialized_hashes(word_vec: Vec<String>) -> Vec<String> {
    let word_hash_vec = hash_word_vec(word_vec);
    let serialized_hashes: Vec<String> = Vec::new();
    for word_hash in word_hash_vec {
        let hash_str = generate_hash_str(word_hash);
        serialized_hashes.push(hash_str);
    }
    serialized_hashes
}