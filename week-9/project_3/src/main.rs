use std::io::Write;

fn main() {
    let names = vec!["Aigbogun Alamba Daudu", "Murtala Afeez Bendu", "Okorocha Calistus Ogbona", "Adewale Jimoh Akanbi", "Osazuwa Faith Etieye"];
    let ministries = vec!["Internal Affairs", "Justice", "Defense", "Power & Steel", "Petroleum"];
    let zones = vec!["South West", "North East", "South South", "South West", "South East"];

    let mut file = std::fs::File::create("project_3.csv").expect("create failed");
    file.write_all(b"S/N,Name of Commissioner,Ministry,Geopolitical Zone\n").expect("write failed");

    for i in 0..names.len() {
        writeln!(file, "{},{},{},{}", i+1, names[i], ministries[i], zones[i]).expect("write failed");
    }
    
    println!("Datasets merged and saved.");
}
