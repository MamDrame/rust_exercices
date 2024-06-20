pub fn remove_letter_sensitive(s: &str, letter: char) -> String {
    s.chars().filter(|&c| c != letter).collect()
}

pub fn remove_letter_insensitive(s: &str, letter: char) -> String {
    s.chars()
        .filter(|&c| !c.eq_ignore_ascii_case(&letter))
        .collect()
}

pub fn swap_letter_case(s: &str, letter: char) -> String {
    s.chars()
        .flat_map(|c| {
            if c.eq_ignore_ascii_case(&letter) {
                if c.is_uppercase() {
                    c.to_lowercase().collect::<Vec<_>>()
                } else {
                    c.to_uppercase().collect::<Vec<_>>()
                }
            } else {
                vec![c]
            }
        })
        .collect()
}
