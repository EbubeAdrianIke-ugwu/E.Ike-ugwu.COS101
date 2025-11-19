fn main() {
    // Using a compound data type: A Vector of Tuples (Name, Years of Experience)
    // You can add more candidates to this list easily.
    let candidates = vec![
        ("Adebayo", 6),
        ("Chinelo", 12),
        ("Daniel", 8),
        ("Nneka", 15),
        ("Mustapha", 10),
    ];

    println!("--- EY Global: Recruitment Experience Tracker ---");

    // iter().max_by_key() finds the element with the highest value in the second position (experience)
    if let Some((name, years)) = candidates.iter().max_by_key(|(_, years)| years) {
        println!("HIRED: {} has the highest experience with {} years.", name, years);
    } else {
        println!("No candidates found.");
    }

}