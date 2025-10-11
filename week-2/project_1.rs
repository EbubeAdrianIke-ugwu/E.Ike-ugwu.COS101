fn main() {
    // Given values
    let principal: f64 = 520_000_000.0; // ₦520,000,000
    let rate: f64 = 10.0;               // 10% per annum
    let years: u32 = 5;                 // 5 years

    // Calculate Amount (A)
    let amount = principal * (1.0 + rate / 100.0).powf(years as f64);

    // Calculate Compound Interest (CI)
    let compound_interest = amount - principal;

    // Display results
    println!("Principal (P): ₦{:.2}", principal);
    println!("Rate (R): {:.2}%", rate);
    println!("Time (n): {} years", years);
    println!("Total Amount (A): ₦{:.2}", amount);
    println!("Compound Interest (CI): ₦{:.2}", compound_interest);
}