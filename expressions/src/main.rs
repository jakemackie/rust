fn main() {
    let number = {
        let x: i32 = 5;
        // Don't include ; at the end of the expression
        x + 1
    };
    println!("{}", number)
}
