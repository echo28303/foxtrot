use blake3::Hasher;

pub fn hash(data: &[u8]) -> [u8; blake3::OUT_LEN] {
    let mut hasher = Hasher::new();
    hasher.update(data);
    let mut output = [0u8; blake3::OUT_LEN];
    output.copy_from_slice(hasher.finalize().as_bytes());
    output
}

