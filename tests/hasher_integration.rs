use rust_rainbowtable::hasher;

#[test]
fn test_serialize_hashes() {
    let word_vec = vec![
        "online123".to_string(),
        "earth616".to_string(),
        "multiverse".to_string(),
    ];
    let expected_vec = vec![
        "online123:a611e58490e1cf681f0dd17f6c76bf98537da365464f3327a6d08fb91777cd0d".to_string(),
        "earth616:e2a7be9cd1f4d39f54f93facefdf99334366396f84dfd7061cb32dccba3c40c2".to_string(),
        "multiverse:556a71b43bb411e3b11b3d7a4c2c11fd7d402643757d371638a4c9c2dfa1b753".to_string()
    ];
    let serialized_hashes_vec = hasher::serialize_hashes(word_vec);
    assert_eq!(serialized_hashes_vec, expected_vec);
}

#[test]
fn test_deserialize_hashes() {
    let serialized_hashes = vec![
        "audi:b51026e4444f98ecdbe1d7cb1f310427a47d7a6e7659b37ce3d00010b09af252".to_string(),
        "mercedes:917ebb3396b2ff2e27b75e3fe421b1edc07b998f74350472f3abc5c6620a68db".to_string(),
        "bmw:27df9ed9a477af0fcfe369c8ef3474a75cebf357d8b421ca40f1de6cfd4cbb06".to_string()
    ];

    let expected_word_hashes = vec![
        hasher::WordHash {
            word: "audi".to_string(),
            hash: "b51026e4444f98ecdbe1d7cb1f310427a47d7a6e7659b37ce3d00010b09af252".to_string()
        },
        hasher::WordHash {
            word: "mercedes".to_string(),
            hash: "917ebb3396b2ff2e27b75e3fe421b1edc07b998f74350472f3abc5c6620a68db".to_string()
        },
        hasher::WordHash {
            word: "bmw".to_string(),
            hash: "27df9ed9a477af0fcfe369c8ef3474a75cebf357d8b421ca40f1de6cfd4cbb06".to_string()
        }
    ];

    let deserialized_hashes = hasher::deserialize_hashes(serialized_hashes);
    match deserialized_hashes {
        Ok(deserialized_hashes) => assert_eq!(expected_word_hashes, deserialized_hashes),
        Err(e) => panic!("{}", e)
    }
}

#[test]
fn test_deserialize_hashes_invalid() {
    let serialized_hashes = vec![
        "audib51026e4444f98ecdbe1d7cb1f310427a47d7a6e7659b37ce3d00010b09af252".to_string(),
        "mercedes:917ebb3396b2ff2e27b75e3fe421b1edc07b998f74350472f3abc5c6620a68db".to_string(),
        "bmw:27df9ed9a477af0fcfe369c8ef3474a75cebf357d8b421ca40f1de6cfd4cbb06".to_string()
    ];
    let result = hasher::deserialize_hashes(serialized_hashes);
    assert!(result.is_err());

    let serialized_hashes = vec![
        "audi:b51026e4444f98ecdbe1d7cb1f310427a47d7a6e7659b37ce3d00010b09af252".to_string(),
        "adwadwad".to_string(),
        "bmw:27df9ed9a477af0fcfe369c8ef3474a75cebf357d8b421ca40f1de6cfd4cbb06".to_string()
    ];
    let result = hasher::deserialize_hashes(serialized_hashes);
    assert!(result.is_err());

    let serialized_hashes = vec![
        "audi:b51026e4444f98ecdbe1d7cb1f310427a47d7a6e7659b37ce3d00010b09af252".to_string(),
        "mercedes:917ebb3396b2ff2e27b75e3fe421b1edc07b998f74350472f3abc5c6620a68db".to_string(),
        "".to_string()
    ];
    let result = hasher::deserialize_hashes(serialized_hashes);
    assert!(result.is_err());
}