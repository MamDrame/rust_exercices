const LETTERS: &str = "abcdefghijklmnopqrstuvwxyz";
pub fn rot21(input: &str) -> String {
    input.chars().map(|ch| {
        if !LETTERS.contains(ch.to_ascii_lowercase()) {
        return ch;
    };
        let position = LETTERS.chars().position(|c| c == ch.to_ascii_lowercase()).unwrap_or_default();
        let corresponding_letter = LETTERS.chars().nth((position + 21) % 26).unwrap_or_default();
        if ch.is_ascii_uppercase() {
            corresponding_letter.to_ascii_uppercase()
        } else {
            corresponding_letter
        }}).collect()
}