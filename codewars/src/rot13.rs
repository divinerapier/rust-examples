pub fn rot13(message: &str) -> String {
    // your code here
    let res = message
        .as_bytes()
        .iter()
        .map(|&v| {
            if v >= b'a' && v <= b'z' {
                b'a' + ((v - b'a') + 13) % 26
            } else if v >= b'A' && v <= b'Z' {
                b'A' + ((v - b'A') + 13) % 26
            } else {
                v
            }
        })
        .collect::<Vec<u8>>();
    unsafe { String::from_utf8_unchecked(res) }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(rot13("test"), "grfg");
        assert_eq!(rot13("Test"), "Grfg");
    }
}
