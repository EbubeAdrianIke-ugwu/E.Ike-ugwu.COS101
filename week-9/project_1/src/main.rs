use std::io::Write;

fn main() {
    let lager = vec!["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"];
    let stout = vec!["Legend", "Turbo King", "Williams"];
    let non_alc = vec!["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"];

    let mut file = std::fs::File::create("project_1.txt").expect("create failed");
    file.write_all(b"Lager,Stout,Non-Alcoholic\n").expect("write failed");

    for i in 0..lager.len() {
        // simple way to pick an item or use empty text if we run out
        let l = lager.get(i).unwrap_or(&"");
        let s = stout.get(i).unwrap_or(&"");
        let n = non_alc.get(i).unwrap_or(&"");

        writeln!(file, "{},{},{}", l, s, n).expect("write failed");
    }
    println!("File created successfully.");
}