use rayon::prelude::*;
use sha2::{Digest, Sha256};
use std::collections::HashMap;

fn par_sha256(data: &HashMap<String, Vec<u8>>) -> HashMap<String, Vec<u8>> {
    data.par_iter()
        .map(|(key, bytes)| {
            let mut hasher = Sha256::new();
            hasher.update(bytes);
            let hash = hasher.finalize();
            (key.to_owned(), hash.to_vec())
        })
        .collect()
}

fn main() {
    let mut data = HashMap::new();

    data.insert("key1".to_string(), b"Hello, World!".to_vec());
    data.insert("key2".to_string(), b"Another example".to_vec());

    let hashed_data = par_sha256(&data);

    for (key, hash) in hashed_data {
        println!("{}: {}", key, hex::encode(hash));
    }
}
