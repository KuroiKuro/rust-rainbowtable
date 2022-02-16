use sha2::{Sha256, Digest};

pub const HASH_DELIMITER: &str = ":";

pub struct WordHash {
    pub word: String,
    pub hash: String,
}

fn hash_word(word: &str) -> String {
    let mut hash = Sha256::new();
    hash.update(word);
    format!("{:x}", hash.finalize())
}

fn hash_word_vec(word_vec: Vec<String>) -> Vec<WordHash> {
    let mut hash_vec: Vec<WordHash> = Vec::new();
    for word in word_vec {
        let hash = hash_word(&word);
        let word_hash = WordHash {
            word: word,
            hash: hash
        };
        hash_vec.push(word_hash);
    }
    hash_vec
}

fn generate_hash_str(word_hash: WordHash) -> String {
    let mut hash_str = String::from(word_hash.word);
    hash_str.push_str(HASH_DELIMITER);
    hash_str.push_str(&format!("{}", word_hash.hash));
    hash_str
}

fn deserialize_single_hash(serialized_hash: String) -> Result<WordHash, String> {
    let split_vec = serialized_hash.split(HASH_DELIMITER).collect::<Vec<&str>>();
    if split_vec.len() != 2 {
        return Err(
            format!("Invalid serialized hash, got: {}", serialized_hash)
        );
    }
    // Improvement for next time: validate that hash is a valid hash
    Ok(
        WordHash {
            word: String::from(split_vec[0]),
            hash: String::from(split_vec[1]),
        }
    )
}

pub fn generate_serialized_hashes(word_vec: Vec<String>) -> Vec<String> {
    let word_hash_vec = hash_word_vec(word_vec);
    let mut serialized_hashes: Vec<String> = Vec::new();
    for word_hash in word_hash_vec {
        let hash_str = generate_hash_str(word_hash);
        serialized_hashes.push(hash_str);
    }
    serialized_hashes
}

pub fn deserialize_hashes(serialized_hashes: Vec<String>) -> Result<Vec<WordHash>, String> {
    let mut deserialized_hashes: Vec<WordHash> = Vec::new();
    for serialized_hash in serialized_hashes {
        deserialized_hashes.push(deserialize_single_hash(serialized_hash)?);
    }
    Ok(deserialized_hashes)
}
