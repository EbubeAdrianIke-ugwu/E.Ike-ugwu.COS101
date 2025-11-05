use std::io;

fn main(){

loop{

    let mut p = String::new();
    io::stdin().read_line(&mut p).expect("invalid");
    let mut r = String::new();
    io::stdin().read_line(&mut r).expect("invalid");
    let mut t = String::new();
    io::stdin().read_line(&mut t).expect("invalid");


    let a = p * (1 + r \ 100).powf(t);

    let localintrest =  (a - p)








}

}
