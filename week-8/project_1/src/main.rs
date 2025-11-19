use std::io;

fn main() {
    // Define data as a list of tuples: ("Level", ["Admin", "Academic", "Lawyer", "Teacher"])
    let database = [
        ("APS 1-2", ["Intern", "-", "Paralegal", "Placement"]),
        ("APS 3-5", ["Administrator", "Research Assistant", "Junior Associate", "Classroom Teacher"]),
        ("APS 5-8", ["Senior Administrator", "PhD Candidate", "Associate", "Snr Teacher"]),
        ("EL1 8-10", ["Office Manager", "Post-Doc Researcher", "Senior Associate 1-2", "Leading Teacher"]),
        ("EL2 10-13", ["Director", "Senior Lecturer", "Senior Associate 3-4", "Deputy Principal"]),
        ("SES", ["CEO", "Dean", "Partner", "Principal"]),
    ];

    // Map the index of the job title to its category name
    let categories = ["Office Administrator", "Academic", "Lawyer", "Teacher"];

    println!("Public Service APS Level Checker (Type 'exit' to quit)");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let job = input.trim();

        if job.eq_ignore_ascii_case("exit") { break; }

        let mut found = false;
        for (level, titles) in &database {
            // iter().position() finds the index of the matching title, if it exists
            if let Some(idx) = titles.iter().position(|t| t.eq_ignore_ascii_case(job)) {
                println!("VALIDATED: Staff is a {} holding position {}", categories[idx], level);
                found = true;
                break;
            }
        }

        if !found { println!("Not Found: '{}'", job); }
    }
}
