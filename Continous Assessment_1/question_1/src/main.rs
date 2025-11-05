use std::io;

fn main() {

    //prompt the user to enter username
    println!("Enter customer name");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("invalid input");
    let _name = name.trim();

    //units consumed
    let mut unit1 = String::new();
    let mut unit2 = String::new();
    let mut unit3 = String::new();

    println!("Enter first unit");
    io::stdin().read_line(&mut unit1).unwrap();
    println!("Enter second unit");
    io::stdin().read_line(&mut unit2).unwrap();
    println!("Enter third unit");
    io::stdin().read_line(&mut unit3).unwrap();



    //convert units to numbers
    let unit1: f64 = unit1.trim().parse().unwrap();
    let unit2: f64 = unit2.trim().parse().unwrap();
    let unit3: f64 = unit3.trim().parse().unwrap();

    //enter 
    let rate =  if unit1 <=100.0 {"20"}
    else if unit1 >=101.0 - 300.0 {"35"}
    else if unit1 >=301.0 {"50"}
    else {"invalid"};

     let rate =  if unit2 <=100.0 {"20"}
    else if unit2 >=101.0 - 300.0 {"35"}
    else if unit2 >=301.0 {"50"}
    else {"invalid"};

 let rate =  if unit3 <=100.0 {"20"}
    else if unit3 >=101.0 - 300.0 {"35"}
    else if unit3 >=301.0 {"50"}
    else {"invalid"};

    let total = (unit1 + unit2 + unit3);



   

   println!("\n---costomer details---");
   println!("Customer name: {} ", name);
   println!("units consumed:{} ", unit1);
   println!("total: {} ",total);




    


    

}
