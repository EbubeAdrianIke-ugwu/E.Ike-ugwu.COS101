use std::io;

fn main() {
    let mut experience = String::new();
    let mut age = String::new();

    // Input experience
    println!("Is the employee experienced? (yes/no):");
    io::stdin().read_line(&mut experience).expect("Failed to read input");
    let experience = experience.trim().to_lowercase();

    // Input age
    println!("Enter the employee's age:");
    io::stdin().read_line(&mut age).expect("Failed to read input");
    let age: u32 = age.trim().parse().expect("Please enter a valid number");

    let incentive: u32;

    if experience == "yes" {
        if age >= 40 {
            incentive = 1_560_000;
        } else if age >= 30 && age < 40 {
            incentive = 1_480_000;
        } else if age < 28 {
            incentive = 1_300_000;
        } else {
            incentive = 1_000_000; // Default for experienced but not fitting above (optional)
        }
    } else {
        incentive = 100_000;
    }

    println!("The employee's annual incentive is: ₦{}", incentive);
}
