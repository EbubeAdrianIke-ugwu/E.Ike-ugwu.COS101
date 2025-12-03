// use std::io::Write

// fn main() {

//     let announce = "Week 9 - Rust File Input & Output\n";
//     let dept = "Department of Computer Science";


//     let mut file = std::fs::File::create("data.txt").expect("create failed");
//     file.write_all("Welcome to Rust Programming\n"
//             .as_bytes()).expect("Write failed");
//     file.write_all(announce.as_bytes()).expect("write failed");
//     file.write_all(dept.as_bytes()).expect("write failed");
//     println!("\nData written to file.")
// }

// use std::io;

// fn main() {
//     println!("Guess the number!");

//     println!("Please input your guess.");

//     let mut guess = String::new();

//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line");

//     println!("You guessed: {}",guess);
// }

use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = 20;

    // --snip--

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        // --snip--

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

       let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => println!("Please input a valid number"),
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
