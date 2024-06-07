//This function receives a number n and returns the nth number in the Lucas Numbers where the nth number is the sum of the previous two numbers in the series.
//
// The Lucas Numbers start like this: 2, 1, 3, 4, 7, 11, 18, 29, 47, 76, 123, etc...
const FIRST_LUCAS: u32 = 2;
const SECOND_LUCAS: u32 = 1;

pub fn lucas_number(lucas_position: u32) -> u32 {
    if lucas_position == 0 {
        FIRST_LUCAS
    } else if lucas_position == 1 {
        SECOND_LUCAS
    } else {
        let mut prev_number = FIRST_LUCAS;
        let mut current_number = SECOND_LUCAS;
        for _ in 2..=lucas_position {
            let temp = prev_number + current_number;
            prev_number = current_number;
            current_number = temp;
        }
        current_number
    }
}