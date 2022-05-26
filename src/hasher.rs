use sha2::{Sha256, Digest};

pub const HASH_DELIMITER: &str = ":";

#[derive(PartialEq, Eq, Debug)]
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
    /*
        Generate a line that contains the word and the hash, delimited by HASH_DELIMITER
    */
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

pub fn serialize_hashes(word_vec: Vec<String>) -> Vec<String> {
    let word_hash_vec = hash_word_vec(word_vec);
    let mut serialized_hashes: Vec<String> = Vec::new();
    // TODO: Refactor to use iterator!!
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_word() {
        let test_word = "myword";
        let expected_hash = "72ba3446a1abd27d95c967079a8c3e79b0fa88dd0dd0c332f8e471683327d8a2";
        let hash = hash_word(test_word);
        assert_eq!(expected_hash, hash);
        
        let test_word = "password12345";
        let expected_hash = "3700adf1f25fab8202c1343c4b0b4e3fec706d57cad574086467b8b3ddf273ec";
        let hash = hash_word(test_word);
        assert_eq!(expected_hash, hash);
    }

    #[test]
    fn test_hash_word_vec() {
        let expected_vec = vec![
            WordHash {
                word: "origami45".to_string(),
                hash: "fa4f4a682bfb7477ca513001ed73d1fd999572174f718ea502d8b86584e44fd8".to_string()
            },
            WordHash {
                word: "nintendo64".to_string(),
                hash: "be2876a1aa8dcfbafc3e5f145b3a572575393a016863ce59e45692d28467e4dd".to_string()
            },
            WordHash {
                word: "KBF8GgQCbWBazt".to_string(),
                hash: "10f8b6f0f46b4d5dda8ceece3d77cffc8951ba202d35aa72aff5ef839fad8c4a".to_string()
            },
        ];
        let word_vec = vec![
            "origami45".to_string(), "nintendo64".to_string(), "KBF8GgQCbWBazt".to_string()
        ];

        let hash_word_vec = hash_word_vec(word_vec);
        assert_eq!(expected_vec, hash_word_vec);
    }

    #[test]
    fn test_generate_hash_str() {
        let word_hash = WordHash {
            word: "zombie".to_string(),
            hash: "49460b7bbbd3aad3f2cba09864f5e8b01a220ea8c077e9fa996de367e7984af0".to_string()
        };
        let expected_string = "zombie:49460b7bbbd3aad3f2cba09864f5e8b01a220ea8c077e9fa996de367e7984af0";
        let hash_str = generate_hash_str(word_hash);
        assert_eq!(expected_string, hash_str);
    }

    #[test]
    fn test_deserialize_single_hash() {
        let serialized_hash = "command:5d347fd948b66308f502c3f65c8f7e12ff1c5cf8c760bcdfb188ae1ec7b8b618";
        let expected_word_hash = WordHash {
            word: "command".to_string(),
            hash: "5d347fd948b66308f502c3f65c8f7e12ff1c5cf8c760bcdfb188ae1ec7b8b618".to_string()
        };
        let deserialized_hash = deserialize_single_hash(serialized_hash.to_string());
        match deserialized_hash {
            Ok(deserialized_hash) => assert_eq!(expected_word_hash, deserialized_hash),
            Err(e) => {
                panic!("{}", e);
            }
        }
    }

    #[test]
    fn test_deserialize_single_hash_invalid() {
        let invalid_str = "abc";
        let deserialized_hash = deserialize_single_hash(invalid_str.to_string());
        assert!(deserialized_hash.is_err());
    }

}
