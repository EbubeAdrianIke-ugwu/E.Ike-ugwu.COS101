fn main() {
    // Define sales amounts
    let amounts = [450000.0, 1500000.0, 750000.0, 2850000.0, 250000.0];

    // Calculate total sum
    let total: f64 = amounts.iter().sum();

    // Calculate average
    let average = total / amounts.len() as f64;

    // Display the results
    println!("=== Sales Record Summary ===");
    println!("Total Sales Amount: ₦{:.2}", total);
    println!("Average Sales Amount: ₦{:.2}", average);
}