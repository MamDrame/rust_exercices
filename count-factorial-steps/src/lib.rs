//Create a function named count_factorial_steps that receives a factorial number and counts how many multiplications are necessary to have this number.
//
// If the argument is not a factorial, or it is equal 0 or 1, then the function should return 0.
//
// pub fn count_factorial_steps(factorial: u64) -> u64 {
// }
// As a reminder, the factorial of a number is the product of all the integers from 1 to that number.
//
// Example: the factorial of 6 (written 6!) is 1 * 2 * 3 * 4 * 5 * 6 = 720.
pub fn count_factorial_steps(factorial: u64) -> u64 {
    if factorial == 0 || factorial == 1 {
        return 0;
    }
    let mut remainder = factorial;
    let mut step = 1;
    let mut divisor = 2;
    while remainder > 1 {
        // Vérifie si 'remainder' est divisible par tous les nombres à partir de 2 jusqu'à 'divisor'
        if remainder % divisor != 0 {
            return 0;
        }
        remainder /= divisor;
        step += 1;
        divisor += 1;
    }
    step
}