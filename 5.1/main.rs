use std::fs;

fn main() {
    star1();
}

fn star1() {
    let inp = String::from(fs::read_to_string("ex.in")
                .expect("Should have been able to read file"));
    
    let (stacks, moves) = inp.split_once("\n\n").unwrap();

    //let nbr_stacks = stacks[].chars().filter(|s| s.is_digit(16)).collect::<Vec<char>>().len();
    // let stacks = 
    // for i in 1..nbr_stacks


    println!("{:#?}", stacks);
    println!("{:#?}", moves);
}