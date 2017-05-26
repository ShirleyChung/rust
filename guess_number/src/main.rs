use std::io;

fn main() {
    println!("Welcome! Guess number");
    
    println!("Come on! enter a number:");
    
    let mut guess = String::new(); // mutable string
    
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line.");
    
    println!("You guess: {}", guess);

}
