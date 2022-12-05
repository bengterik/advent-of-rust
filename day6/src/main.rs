use std::{fs, iter};

fn main() {
    let inp = String::from(fs::read_to_string("in.in")
                .expect("Should have been able to read file"));
 
    println!("{:#?}", inp);
}
