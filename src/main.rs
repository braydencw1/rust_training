use std::io;

fn main() {
    println!("Please enter something for me to return:");
    let mut promt_d = String::new();

    io::stdin().read_line(&mut promt_d)
        .expect("Failed to input");

    println!("Your response {}", promt_d);
}
