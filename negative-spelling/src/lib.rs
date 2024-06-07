pub fn negative_spell(n: i64) -> String {
    if n > 0 {
        return "error: positive number".to_string();
    }
    if n == 0 {
        return "zero".to_string();
    }

    let mut number = -n; // convert to positive for easier processing

    let words = vec![
        "","one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen",
    ];

    let tens_words = vec![
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];

    let mut result = String::from("minus ");

    if number >= 1_000_000 {
        result.push_str("one million");
        number -= 1_000_000;
    }

    if number >= 1000 {
        let thousands = number / 1000;
        result.push_str(&spell_under_1000(thousands, &words, &tens_words));
        result.push_str(" thousand ");
        number %= 1000;
    }

    if number > 0 {
        result.push_str(&spell_under_1000(number, &words, &tens_words));
    }

    result.trim().to_string()
}

fn spell_under_1000(number: i64, words: &[&str], tens_words: &[&str]) -> String {
    let mut result = String::new();

    if number >= 100 {
        let hundreds = number / 100;
        result.push_str(words[hundreds as usize]);
        result.push_str(" hundred ");
    }

    let remainder = number % 100;
    if remainder >= 20 {
        let tens = remainder / 10;
        result.push_str(tens_words[tens as usize]);
        if remainder % 10 != 0 {
            result.push_str("-");
            result.push_str(words[(remainder % 10) as usize]);
        }
    } else if remainder > 0 {
        result.push_str(words[remainder as usize]);
    }

    result.trim().to_string()
}
