fn main() {

    let s1 = "Electrical".to_string();
    let s2 = " Electronic".to_string();
    let s3 = s1 + &s2; // s1 is moved here and can no longer be used
    let s4 = s3 + &s2; // s2 & s3 reference is passed
    println!("{}",s4);

    // About Electrical/Electronic
    println!("The {} is informed by the aspiration to train electrical/electronic engineering professionals in the areas of design, building and maintenance of electrical control systems.", s4);

    let w1 = "Computer".to_string();
    let w2 = " Science".to_string();
    let w3 = w1 + &w2; // w2 reference is passed
    println!("{}", w3);

    println!("{} is aimed at developing competent, creative, innovative, entrepreneurial and ethically-minded persons, capable of creating value in the diverse fields of Computer Science.", w3);

}
