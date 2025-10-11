fn main() {
    // Given values
    let p: f64 = 510000.0; // initial cost in Naira
    let r: f64 = 5.0;      // depreciation rate per annum
    let n: f64 = 3.0;      // number of years

    // Apply depreciation formula: A = P * (1 - R/100)^n
    let a = p * (1.0 - r / 100.0).powf(n);

    // Calculate total depreciation
    let depreciation = p - a;

    // Display results
    println!("=== Depreciation Calculation ===");
    println!("Original Value: ₦{:.2}", p);
    println!("Rate of Depreciation: {:.2}%", r);
    println!("Number of Years: {}", n);
    println!("Value After {} Years: ₦{:.2}", n, a);
    println!("Total Depreciation: ₦{:.2}", depreciation);
}