pub fn encode_i64(n: i64) -> Vec<u8> {
    n.to_le_bytes().to_vec()
}

pub fn encode_string(s: &str) -> Vec<u8> {
    let mut bytes = vec![s.len() as u8];
    bytes.extend_from_slice(s.as_bytes());

    bytes
}

pub fn encode_usize(n: usize) -> Vec<u8> {
    n.to_le_bytes().to_vec()
}