use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    println!("* Guess number game *");
    
    let secret_number = rand::thread_rng().gen_range(1, 101);
    
    loop {
        println!("Enter your number:");
    
        let mut guess = String::new();
    
        io::stdin().read_line(&mut guess)
            .expect("Cant read line.");
    
        let guess: u32 = guess.trim().parse()
            .expect("Please ENTER number.");
    
        println!("Secret number is: {}", secret_number);
    
        println!("You set number: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too less..."),
            Ordering::Greater => println!("To large..."),
            Ordering::Equal => {
                println!("YOU WON!!!");
                break;
            } 
        }
    }
}
