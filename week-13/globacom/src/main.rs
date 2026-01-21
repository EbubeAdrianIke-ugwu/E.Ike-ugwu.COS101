use std::io;

// Define structures for each table
struct Customer {
    c_id: u32,
    c_name: String,
    c_age: u32,
    c_email: String,
    c_mobile: String,
    e_id: u32,
    data_id: u32,
}

struct Dataplan {
    data_id: u32,
    data_size: String,
    data_duration: u32,
    data_price: u32,
}

struct Employee {
    e_id: u32,
    e_name: String,
    e_position: String,
    e_email: String,
}

struct Project {
    p_id: u32,
    p_name: String,
    p_duration_days: u32,
}

// Functions to show structures
fn customer_table() {
    println!("Customer Table:");
    println!("C_id  |  C_name  |  C_age  |  C_email  |  C_mobile  |  E_id  |  Data_id");
}

fn dataplan_table() {
    println!("Dataplan Table:");
    println!("Data_id  |  Data_size  |  Data_duration (days)  |  Data_price (naira)");
}

fn employee_table() {
    println!("Employee Table:");
    println!("E_id  |  E_name  |  E_position  |  E_email");
}

fn project_table() {
    println!("Project Table Structure:");
    println!("P_id  | P_name  |  P_duration_days");
}

fn database_structure() {
    println!("Full Database Structure:");
            customer_table();
            dataplan_table();
            employee_table();
            project_table();
}

fn main() {
    println!("Enter your role (admin/project_manager/employee/customer/vendor):");

    let mut role = String::new();
    io::stdin()
        .read_line(&mut role)
        .expect("Failed to read input");
    let role = role.trim().to_lowercase();

    match role.as_str() {
        "admin" => database_structure(),
        "project_manager" => project_table(),
        "employee" => employee_table(),
        "customer" => customer_table(),
        "vendor" => dataplan_table(),
        _ => println!("Invalid role!"),
    }
}