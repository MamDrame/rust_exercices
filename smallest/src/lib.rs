use std::collections::HashMap;

pub fn smallest(h: HashMap<&str, i32>) -> i32 {
    match h.values().min() {
        Some(min) => *min,
        None => i32::MAX,
    }
}
