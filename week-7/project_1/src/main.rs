use std::io;

// Reusable function to get input
fn input(msg: &str) -> f64 {
    println!("{}", msg);
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed");
    s.trim().parse().unwrap_or(0.0)
}

fn main() {
    println!("1.Trapezium 2.Rhombus 3.Parallelogram 4.Cube 5.Cylinder");
    let choice = input("Select equation (1-5):") as i32;

    if choice == 1 {
        let h = input("Height:");
        let b1 = input("Base 1:");
        let b2 = input("Base 2:");
        println!("Area: {}", h / 2.0 * (b1 + b2));
    } else if choice == 2 {
        println!("Area: {}", 0.5 * input("Diag 1:") * input("Diag 2:"));
    } else if choice == 3 {
        println!("Area: {}", input("Base:") * input("Altitude:"));
    } else if choice == 4 {
        println!("Area: {}", 6.0 * input("Side:").powf(2.0));
    } else if choice == 5 {
        let r = input("Radius:"); 
        let h = input("Height:");
        println!("Volume: {}", std::f64::consts::PI * r.powf(2.0) * h);
    } else {
        println!("Invalid choice");
    }
}
