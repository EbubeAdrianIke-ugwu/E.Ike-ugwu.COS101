// use std::io::Write;


// fn main() {

//     let announce = "week 9 - Rust File Input and Output \n";
//     let dept = "Department of Computer Science";

//     let mut file = std::fs::File::create("data.txt").expect("create failed");
//     file.write_all("Welcome to Rust Programming\n".as_bytes()).expect("write failed") ;
//     file.write_all(announce.as_bytes()).expect("write failed");
//      file.write_all(dept.as_bytes()).expect("write failed");
//      println!("\nData written to file.");

// }

// use std::io::Read;

// fn main() {
//     let mut file = std::fs::File::open("welcome_message.txt").unwrap();
//     let mut contents = String::new();
//     file.read_to_string(&mut contents).unwrap();
//     print!("{}",contents );
// }

// fn main() {
//     std::fs::remove_file("data.txt").expect("could not remove the file");
//     println!("file is removed");
// }

// use std::fs::OpenOptions;
// use std::io::Write;

// fn main() {
//     let mut file = OpenOptions::new().create(true).append(true).open("data.txt").expect("can't open the file");
//     file.write_all("\nhello Class".as_bytes()).expect("write failed");
//     file.write_all("\nThis is the appendage to the document".as_bytes()).expect("write failed");
//     println!("file append success");
// }
 
// use std::io::Write;

// fn main() {
//     let larger = vec!["33 Export", "Desperados","Goldberg", "Guilder", "Heineken", "Star"];
//     let stout = vec!["Legend", "Turbo King", "Williams"];
//     let non_alcoholic = vec!["Maltina", "Amstel Malta", "Malta Gold", "Fayouz"];

//     let mut file = std::fs::File::create("Nigeria_Brewery.txt").expect("create failed");
//     file.write_all("Larger\t\tStout\t\tNon-Alcoholic\n".as_bytes()).expect("write failed");
    
//     for i in 0..larger.len(){
//         let larger_item = larger[i];

//         let stout_item = if i < stout.len(){stout[i]}
//         else{""};

//         let non_alcoholic_item = if i < non_alcoholic.len(){non_alcoholic[i]}
//         else{""};
    
//      let line = format!("{}\t{}\t{}\n", larger_item, stout_item, non_alcoholic_item);
    

//      file.write_all(line.as_bytes()).expect("write failed");

//      println!("Data saved successfully to Nigeria_brewey.txt");
//     };

// }

use std::io::Write;

fn main() {

    let student_name = vec!["Oluchi Mordi","Adams Aliyu", "Shania Bolade","Adekunle Gold","Blanca Edemoh"];
    let matric_number = vec!["ACC10211111","ECO10110101","CSC10328828","EEE11020202","MEE10202001"];
    let department = vec!["Accounting","Economics","Computer","Electrical","Mechanical"];
    let level = vec!["300", "200","100","200","200","100"];

    let mut file = std::fs::File::create("PAU-SIMS.txt").expect("create failed");
    file.write_all("Student Name\t\tMatric Number\t\tDepartment\t\tLevel\n".as_bytes()).expect("write failed");

    for i in 0..student_name.len(){
        let std_name = student_name[i];
        let matric_number_item = [i];
        let department_item = department[i];
        let level_item = [i];

        let line = format!("{}\t{}\t{}\t{}\n", std_name, matric_number_item, department_item, level_item);

        file.write_all(line.as_bytes()).expect("write failed");

        println!("Data saved successfully to PAU-SIMS.txt");


    }


}




