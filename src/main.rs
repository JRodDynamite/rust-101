// Take a number from user and print it
use rand::Rng;


fn main() {
    // take number from user
    println!("Enter a number: ");
    let mut number: String = String::new();
    std::io::stdin().read_line(&mut number).expect("Failed to read line");
    let number: i32 = number.trim().parse().expect("Please type a number!");
    println!("You entered: {}", number);
    let random_number = rand::thread_rng().gen_range(1..=100);
    if number == random_number {
        println!("You guessed it right!");
    } else {
        println!("You guessed it wrong! The number was: {}", random_number);
    }
}
