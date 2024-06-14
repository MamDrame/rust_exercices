pub fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }

    for i in 2..=(n as f64).sqrt() as u64 {
        if n % i == 0 {
            return false;
        }
    }

    true
}

pub fn prev_prime(nbr: u64) -> u64 {
    if nbr == 2 || nbr == 1 {
        return 0;
    }
    
    let mut n =  if let Some(result) = nbr.checked_sub(1) {
        result
    } else {
        0
    };
 
    while n >= 2 {
        if is_prime(n) {
            return n;
        }
        n -= 1;
    }
    nbr
}
