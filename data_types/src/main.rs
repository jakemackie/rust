fn main() {
    let name: &str = "Jake";
    let age: u8 = 18;
    let height: f32 = 6.0;

    let is_teenager: bool = age > 12 && age < 20;

    if is_teenager {
        println!(
            "{} is a teenager ({}), he is {} feet tall",
            name, age, height
        );
    } else {
        println!(
            "{} is not a teenager ({}), he is {} feet tall",
            name, age, height
        );
    }
}
