pub fn parse_hex_bytes(input: &str) -> Option<Vec<u8>> {
    
    if input.is_empty(){
        return Some(Vec::new());
    }
    
    input
    .split_whitespace()
    .map(|tok| u8::from_str_radix(tok, 16).ok())
    .collect()
}