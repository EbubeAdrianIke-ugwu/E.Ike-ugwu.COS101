use std::io::{self, Write};

/// Clears the console screen.
fn clear_screen() {
    // A simple way to clear the console.
    // '\x1B[2J' is an ANSI escape code to clear the screen.
    // '\x1B[H' moves the cursor to the top-left corner.
    print!("\x1B[2J\x1B[H");
    io::stdout().flush().unwrap();
}

/// Displays the cafe menu.
fn display_menu() {
    println!("===================================");
    println!("    Welcome to PAU Café! ☕");
    println!("===================================");
    println!("Code | Item      | Price (₦)");
    println!("-----------------------------------");
    println!(" T   | Tea       | 800");
    println!(" C   | Coffee    | 1,200");
    println!(" S   | Sandwich  | 2,000");
    println!(" J   | Juice     | 1,500");
    println!("-----------------------------------");
    println!("Type 'exit' at any time to finish your order.");
    println!();
}

/// Gets a trimmed line of input from the user.
fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    // We must flush stdout to make sure the prompt appears before read_line
    io::stdout().flush().expect("Failed to flush stdout");
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
        
    input.trim().to_uppercase()
}

fn main() {
    let mut total_cost: u32 = 0;

    // Loop for multiple entries (Task 6)
    loop {
        clear_screen();
        display_menu(); // Task 1: Display the cafe menu
        
        println!("Current Total: ₦{}", total_cost);
        println!("-----------------------------------");

        // Task 2: Ask for item code
        let item_code = get_input("Enter item code (T, C, S, J) or 'exit': ");

        // Check for exit condition
        if item_code == "EXIT" {
            break; // Exit the loop
        }

        // Task 2: Ask for quantity
        let quantity_str = get_input("Enter quantity: ");
        
        // Parse the quantity, skip if invalid
        let quantity: u32 = match quantity_str.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid quantity. Please enter a number.");
                // Pause for 2 seconds to show the error
                std::thread::sleep(std::time::Duration::from_secs(2));
                continue; // Skip to the next loop iteration
            }
        };

        // Get the price based on the code
        let price_per_item = match item_code.as_str() {
            "T" => 800,
            "C" => 1200,
            "S" => 2000,
            "J" => 1500,
            _ => {
                println!("Invalid item code '{}'. Please try again.", item_code);
                std::thread::sleep(std::time::Duration::from_secs(2));
                continue; // Skip to the next loop iteration
            }
        };

        // Add to the total cost
        total_cost += price_per_item * quantity;
    }

    // Loop has ended, now compute final cost
    clear_screen();
    println!("===================================");
    println!("        Order Summary");
    println!("===================================");
    
    // Task 3: Compute total cost (already done as `total_cost`)
    println!("Subtotal: ₦{}", total_cost);

    let mut final_amount = total_cost as f64;

    // Task 4: If total > ₦5,000, apply a 5% discount
    if total_cost > 5000 {
        let discount = final_amount * 0.05;
        final_amount -= discount;
        println!("Discount (5%): -₦{:.2}", discount);
    } else {
        println!("Discount: ₦0.00");
    }

    println!("-----------------------------------");
    // Task 5: Display the final amount
    println!("Final Amount Due: ₦{:.2}", final_amount);
    println!("===================================");
    println!("\nThank you for your order!");
}
