use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Simple Game of Guessing number written in rust");
    println!("Enter the number");

    /* rand::thread_rng gives random number and gen_range specifies the range of random number
     * gen_range(1,10) generates between 1 to 9
     */
    let secret_number = rand::thread_rng().gen_range(1, 10);
   
    println!("The secret Number is {}", secret_number);

    loop{
    
      /* Create a variable /guess/ which is mutable for changing it dynamically which is empty string. 
     * The string argument needs to be mutable so the method can change the string’s content by adding the user input.
    */
     let mut guess = String::new();
     
     /* Input as stdin(std::io::stdin) called readline into std input and variable guess 
     * .expect("Failed to read line"); handles error when wrong input
     */
    io::stdin().read_line(&mut guess).expect("Failed to read Line");
    
    /* guess is again created which is a shadow variable with new one. Main reason for this is
     * When we want to convert variable from one type to another type. Shadowing let us reuse variable name
     * guess.trim().parse(), the guess is the old string, trim helps to remove whitespace at begining and the end.
     * For eg. if user enters 10 and tap enter then it will be "5\n" with this,the \n is eliminated.
     * parse() parses string to numbers.
     * guess: u32 denotes unsigned 32 bit integer.Here Rust will assume secret_number is also u32
     * We use match as we used Ordering below, parse sucessful then ok else err in this case it will skip and take next input
     */
     let guess: u32 = match guess.trim().parse()
    {
         Ok(num) => num,
         Err(_) => continue,
         };
     println!("You guessed: {}", guess);

    /* Ordering has three things Less, Greater, Equal these are possible outputs
     * Comapres the guess value with Secret Number and outputs the result.
    */
    match guess.cmp(&secret_number)
        {
        Ordering::Less => println!("Small"),
        Ordering::Greater => println!("Big"),
        Ordering::Equal =>
                {
                println!("Equal");
                break;
                }
        }
    }   
}