fn rot26(text: &str) -> String {
    let letters = "abcdefghijklmnopqrstuvwxyz";
    let mut output = String::new();

    for ch in text.chars() {
        if !letters.contains(ch.to_ascii_lowercase()) {
            output.push(ch);
            continue;
        };

        let position = letters
            .chars()
            .position(|c| c == ch.to_ascii_lowercase())
            .unwrap_or_default();
        let corresponding_letter = letters
            .chars()
            .nth((25 - position) % 26)
            .unwrap_or_default();

        if ch.is_ascii_uppercase() {
            output.push(corresponding_letter.to_ascii_uppercase());
        } else {
            output.push(corresponding_letter);
        }
    }

    output
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CipherError {
    pub validation: bool,
    pub expected: String,
}

impl CipherError {
    pub fn new(validation: bool, expected: String) -> CipherError {
        Self {
            validation,
            expected,
        }
    }
}

pub fn cipher(original: &str, ciphered: &str) -> Option<Result<bool, CipherError>> {
    if original.is_empty() || ciphered.is_empty() {
        return None;
    };

    

    let expected = rot26(original);
    if expected == ciphered.to_string() {
        return Some(Ok(true));
    } else {
        return Some(Err(CipherError::new(false, expected)));
    }
}
