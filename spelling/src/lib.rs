

pub fn spell(n: u64) -> String {
    match n {
        0 => "zero".to_string(),
        1..=19 => small_numbers(n),
        20..=99 => tens(n),
        100..=999 => hundreds(n),
        1000..=999999 => thousands(n),
        1000000 => "one million".to_string(),
        _ => unimplemented!("The function is not designed to handle numbers beyond one million."),
    }
}

fn small_numbers(n: u64) -> String {
    let words = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
        "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen",
        "seventeen", "eighteen", "nineteen"];
    words[(n - 1) as usize].to_string()
}

fn tens(n: u64) -> String {
    let tens = ["twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"];
    if n < 20 {
        return small_numbers(n);
    }
    let prefix = tens[(n / 10 - 2) as usize];
    let remainder = n % 10;
    if remainder == 0 {
        prefix.to_string()
    } else {
        format!("{}-{}", prefix, small_numbers(remainder))
    }
}

fn hundreds(n: u64) -> String {
    let hundreds = n / 100;
    let remainder = n % 100;
    let prefix = format!("{} hundred", small_numbers(hundreds));
    if remainder == 0 {
        prefix
    } else {
        format!("{} {}", prefix, if remainder < 20 { small_numbers(remainder) } else { tens(remainder) })
    }
}

fn thousands(n: u64) -> String {
    let thousands = n / 1000;
    let remainder = n % 1000;
    let prefix = format!("{} thousand", spell(thousands));
    if remainder == 0 {
        prefix
    } else if remainder < 100 {
        format!("{} {}", prefix, tens(remainder))
    } else {
        format!("{} {}", prefix, hundreds(remainder))
    }
}
