// Entry point
fn main() {
    let result = add_numbers(5, 10);
    println!("{}", result);
}

// The arrow operator -> is used to indicate the return type of the function
fn add_numbers(x: i32, y: i32) -> i32 {
    // Like expressions, don't include ; at the end of a function that returns a value
    let sum = x + y;

    if sum > 10 {
        println!("The sum is greater than 10");
    } else {
        println!("The sum is less than 10");
    }
    // sum is returned
    sum
}
