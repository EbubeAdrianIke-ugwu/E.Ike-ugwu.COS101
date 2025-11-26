use std::io::Write;

struct Student {
    name: String,
    matric: String,
    dept: String,
    lvl: u32,
}

fn main() {
    let students = vec![
        Student { name: "Oluchi Mordi".to_string(), matric: "ACC10211111".to_string(), dept: "Accounting".to_string(), lvl: 300 },
        Student { name: "Adams Aliyu".to_string(), matric: "ECO10110101".to_string(), dept: "Economics".to_string(), lvl: 100 },
        Student { name: "Shania Bolade".to_string(), matric: "CSC10328828".to_string(), dept: "Computer".to_string(), lvl: 200 },
        Student { name: "Adekunle Gold".to_string(), matric: "EEE11020202".to_string(), dept: "Electrical".to_string(), lvl: 200 },
        Student { name: "Blanca Edemoh".to_string(), matric: "MEE10202001".to_string(), dept: "Mechanical".to_string(), lvl: 100 },
    ];

    let mut file = std::fs::File::create("project_2.csv").expect("create failed");
    file.write_all(b"Student Name,Matric Number,Department,Level\n").expect("write failed");

    for s in students {
        // print to console
        println!("Saving: {} - {}", s.name, s.matric);
        // save to file
        writeln!(file, "{},{},{},{}", s.name, s.matric, s.dept, s.lvl).expect("write failed");
    }
    println!("Data saved.");
}
