pub fn talking(text: &str) -> &str {
    if text.trim().is_empty() {
        return "Just say something!";
    }

    let aphabet = text.chars().any(|c| c.is_alphabetic());
    let yelling = text.chars().filter(|c| c.is_alphabetic()).all(|c| c.is_ascii_uppercase());
    let question = text.ends_with("?");

    if aphabet && yelling && !question {
        return "There is no need to yell, calm down!";
    } else if aphabet && yelling && question {
        return "Quiet, I am thinking!"
    } else if question {
        return "Sure."
    };
    "Interesting"
}