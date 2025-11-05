fn main() {
    let a:i32 = 2; // Bit presentation 10
    let B:i32 = 3; // Bit presentation 11

    let mut result:i32;

    result = a & B;
    println!("(a & B) => {}", result);

    result = a | B;
    println!("(a | B) => {}", result);

    result = a ^ B;
    println!("(a ^ B) => {}", result);

    result = a << B;
    println!("(a << B) => {}", result);

    result = a >> B;
    println!("(a >> B) => {}", result);
}
