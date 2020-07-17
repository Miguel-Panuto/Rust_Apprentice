fn main() {
    let s = String::from("Hello World");

    takes_string(s);


    let x = 2;
    takes_number(x);
    println!("{}", x);
}


fn takes_string(s: String) {
    println!("{}",s);
}

fn takes_number(x: u8) {
    println!("{}", x); 
}