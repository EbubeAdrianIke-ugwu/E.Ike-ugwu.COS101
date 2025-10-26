use std::io;
use std::f64;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    println!("Enter the value of a:");
    io::stdin().read_line(&mut a).expect("Failed to read input");
    let a: f64 = a.trim().parse().expect("Please enter a valid number");

    println!("Enter the value of b:");
    io::stdin().read_line(&mut b).expect("Failed to read input");
    let b: f64 = b.trim().parse().expect("Please enter a valid number");

    println!("Enter the value of c:");
    io::stdin().read_line(&mut c).expect("Failed to read input");
    let c: f64 = c.trim().parse().expect("Please enter a valid number");

    let discriminant = b * b - 4.0 * a * c;

    if discriminant > 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("Two distinct real roots: {:.2} and {:.2}", root1, root2);
    } else if discriminant == 0.0 {
        let root = -b / (2.0 * a);
        println!("One real root: {:.2}", root);
    } else {
        let real_part = -b / (2.0 * a);
        let imag_part = (-discriminant).sqrt() / (2.0 * a);
        println!(
            "Complex roots: {:.2} + {:.2}i and {:.2} - {:.2}i",
            real_part, imag_part, real_part, imag_part
        );
    }
}


