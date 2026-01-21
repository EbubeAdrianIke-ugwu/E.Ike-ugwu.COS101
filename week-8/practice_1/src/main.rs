// fn main() {

//     // Using Vec::new()
//     let v : Vec<i64> = Vec::new();

//     // printing the size of vector
//     println!("\nThe length of Vec::new is: {}",v.len());

//     // Using macro
//     let v = vec!["Adrian","Igwe", "Gerald", "Emeka", "Mac-Anthony"];

//     // printing the size of vector
//     println!("\nThe length of vec macro is: { }",v.len());

// }

// fn main() {
//     let condition = true;
//     let number = if condition { 6} else { 0 };

//     println!("The value of number is: {number}");
// }

// fn main() {
//     let  goat = "Neymar";
//     loop {
//         if goat == "Messi"{
//             println!("again!");
//         } else {
//             println!("Ronaldo is the goat" );
//         };
//         break;
//     }
// }

// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("The result is {result}");
// }

// fn main() {
//     {                      // s is not valid here, since it's not yet declared
//         let s = "hello";   // s is valid from this point forward

//         // do stuff with s
//         println!("the value of s is {}",s ); // this scope is now over, and s is no longer valid
//     }  
//     println!("this is {}", s);                    
// }

// fn main() {
//     let mut wood: i32 = 35;
//     bush(&mut wood);
//     wood = wood * 2;
//     println!("The value of wood is: {}", wood);
// }

// fn bush(plank: &mut i32) {
//     *plank = plank / 5;
//     let wood = *plank / 3;
//     println!("The value of plank is: {}", plank);
    
// }
// fn main() {
//     let first = "Santa Claus".to_string();
//     let noel = &first[3..11];
//     println!("{}", noel);
// }
// fn main() {
//     let data = ["Ade", "Ola", "Joye", "Adam", "Yemi", "Sam", "Tola"];
//     pass_me(&data[4..]);
// }

// fn pass_me(use_data: &[&str]) {
//     println!("The length of use_data is: {:?}", use_data.len());
//     println!("{:?}", use_data);
// }
// fn main() {
//     for magic_key in 20..29 {
//         if magic_key <= 25 {
//             continue;
//         }
//         println!("key is {}", magic_key - 3);
//     }
// }
// fn main() {
//     let mut lab = 15;
//     let mut class = 50;
//     let mut min = 4;
//     let mut max = 7;

//     while lab < class {
//         lab += min;
//         class -= max;
//         println!("The value of class = {}", class);
//     }
// }
// fn main() {
//     for x in 29..31 {
//         for mut m in 15..17 {
//             m += 3;
//             let z = m + x;
//             println!("The value of z is {}\n", z);
//         }
//     }
// }

// fn main() {
//     for num1 in 8..10 {
//         for num2 in 16..17 {
//             for num3 in 15..17 {
//                 let val = num1 + num2 + num3;
//                 println!("{}", val);
//             }
//         }
//     }
// }
fn main() {
    let mut game: i32 = 25;
    let mut two: i32 = 15;

    if game > 0 {
        game += 3;
        two -= 2;

        let grass = game + two;
        game = grass / 2;
        two = game * 3;
        
        println!("grass, game and two are {}, {}, {}", grass, game, two);
    }
}