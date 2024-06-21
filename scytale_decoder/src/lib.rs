pub fn scytale_decoder(s: String, letters_per_turn: u32) -> Option<String> {
    if s.is_empty() || letters_per_turn == 0 {
        return None;
    }

    let letters_per_turn = letters_per_turn as usize;
    let length = s.len();
    let rows = (length + letters_per_turn - 1) / letters_per_turn; // Ceiling division

    let mut decoded_chars = vec![' '; length];
    let chars: Vec<char> = s.chars().collect();

    for i in 0..letters_per_turn {
        for j in 0..rows {
            let decoded_index = i + j * letters_per_turn;
            if decoded_index < length {
                decoded_chars[j + i * rows] = chars[decoded_index];
            }
        }
    }

    Some(decoded_chars.into_iter().collect())
}