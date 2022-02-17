# Rust Rainbowtable
A simple project to learn Rust! Inspired by a similar project I had to do in school using C and linked lists.

# Purpose
This program has 2 functions, `generate_table` and `crack_hash`

`generate_table` allows the user to input a wordlist, and a rainbow table consisting of SHA256 hashes will be generated for the words in the wordlist, and serialized to a specified filepath

`crack_hash` allows a user to input a serialized rainbow table file and a hash, and the program will load the serialized hashes into a vector stored in memory and try to crack the hash by attempting to find the hash and returning the corresponding word

# Usage
This project was started with `cargo` so we can build it by using `cargo build --release`.

## Generate Table
```
./rust-rainbowtable generate_table RAINBOW_TABLE_FILE_PATH WORD_LIST_FILE_PATH
```
## Crack Hash
```
./rust-rainbowtable crack_hash RAINBOW_TABLE_FILE_PATH HASH
```

# Future Improvements
* Check that hashes in rainbow table file are valid hashes
* Improve CLI argument parsing (currently quite basic)
* Support more hashes
* Implement multi-threading (can possibly split the vector into multiple vectors and assign one vector to one thread)
* Cleanup code
