use std::io;

fn main() {
    println!("Simple Game of Guessing number written in rust");
    println!("Enter the number");
    /* Create a variable /guess/ which is mutable for changing it dynamically which is empty string. 
     * The string argument needs to be mutable so the method can change the string’s content by adding the user input.
    */
    let mut guess = String::new();
    /* Input as stdin(std::io::stdin) called readline into std input and variable guess 
     * .expect("Failed to read line"); handles error when wrong input
     */
    io::stdin().read_line(&mut guess).expect("Failed to read Line");
    println!("You guessed: {}", guess);

}
