pub fn capitalize_first(input: &str) -> String {
    let mut output = input.to_string();
    match output.get_mut(0..1).map(|v| v.make_ascii_uppercase()) {
        Some(_) => output,
        None => String::new()
    }
}

pub fn title_case(input: &str) -> String {
    input.split_whitespace().map(|word| {
        capitalize_first(word)
    }).collect::<Vec<String>>().join(" ")
}

pub fn change_case(input: &str) -> String {
    let mut a = String::new();

    for c in input.chars() {
        if c.is_ascii_lowercase() {
            a.push(c.to_ascii_uppercase());
        } else {
            a.push(c.to_ascii_lowercase());
        }
    };
    a
}