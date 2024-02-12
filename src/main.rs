fn main() {
    let name: &str = "Jake";
    let age: i32 = 18;
    let teenager: bool = true;
    let height: f32 = 6.0;

    let is_teenager: &str;

    if teenager {
        is_teenager = "I am a teenager";
    } else {    
        is_teenager = "I am not a teenager";
    }

    println!("My name is {}. {} and I am {} years old. I am {} feet tall.", name, is_teenager, age, height);
}
