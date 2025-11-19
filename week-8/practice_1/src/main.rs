fn main() {

    // Using Vec::new()
    let v : Vec<i64> = Vec::new();

    // printing the size of vector
    println!("\nThe length of Vec::new is: {}",v.len());

    // Using macro
    let v = vec!["Adrian","Igwe", "Gerald", "Emeka", "Mac-Anthony"];

    // printing the size of vector
    println!("\nThe length of vec macro is: { }",v.len());

}