use std::io;

fn main() {
    println!("--- MENU ---");
    println!("P: Pounded Yam / Edinkaiko Soup - ₦3200");
    println!("F: Fried Rice & Chicken - ₦3000");
    println!("A: Amala & Ewedu Soup - ₦2500");
    println!("E: Eba & Egusi Soup - ₦2000");
    println!("W: White Rice & Stew - ₦2500");
    println!("(Enter X to finish)\n");

    let mut total = 0.0;

    loop {
        let mut code = String::new();
        print!("Enter food code: ");
        let _ = io::Write::flush(&mut io::stdout());
        io::stdin().read_line(&mut code).unwrap();
        let code = code.trim().to_uppercase();

        if code == "X" {
            break;
        }

        let price = match code.as_str() {
            "P" => 3200.0,
            "F" => 3000.0,
            "A" => 2500.0,
            "E" => 2000.0,
            "W" => 2500.0,
            _ => {
                println!("Invalid choice!");
                continue;
            }
        };

        let mut qty = String::new();
        print!("Enter quantity: ");
        let _ = io::Write::flush(&mut io::stdout());
        io::stdin().read_line(&mut qty).unwrap();
        let qty: f64 = qty.trim().parse().unwrap_or(0.0);

        total += price * qty;
    }

    if total > 10_000.0 {
        total *= 0.95; // apply 5% discount
        println!("5% discount applied!");
    }

    println!("\nTotal amount: ₦{:.2}", total);
    println!("Thank you!");
}

