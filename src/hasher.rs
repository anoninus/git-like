use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read},
};

use sha2::{Digest, Sha256};
pub fn hasher(file: File, size: u64) -> Result<[u8; 32], Box<dyn Error>> {
    let mut hasher = Sha256::new();
    let header = format!("blob {}\0", size);
    hasher.update(header.as_bytes());
    let mut reader = BufReader::new(file);
    let mut buf = [0u8; 8192];

    loop {
        let bytes_read = reader.read(&mut buf)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buf[..bytes_read]);
    }

    let result = hasher.finalize();
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&result);
    Ok(hash)
}
