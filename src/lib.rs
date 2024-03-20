pub fn square_of_sum(n: u32) -> u32 {
    let mut sum = 0;
    for i in 1..=n {
        sum += i;
    }
    sum * sum
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut sum = 0; // Initialize sum variable outside of the loop
    for i in 1..=n {
        let square = i * i;
        sum += square; // Add square to sum
    }
    sum // Return sum at the end of the function
}

pub fn difference(n: u32) -> u32 {
    let difference = square_of_sum(n) - sum_of_squares(n);
    return difference;
}
