use std::fs;

fn main() {
    star1();
}

fn star1() {
    let inp = String::from(fs::read_to_string("ex.in")
                .expect("Should have been able to read file"));
           
    let stacks = inp.split("\n").filter(|s| !s.starts_with("m")).collect::<Vec<&str>>();
    let moves = inp.split("\n").filter(|s| s.starts_with("m")).collect::<Vec<&str>>();

    let nbr_stacks = stacks[stacks.len()-2].chars().filter(|s| s.is_digit(16)).collect::<Vec<char>>().len();
    // let stacks = 
    // for i in 1..nbr_stacks


    println!("{:#?}", nbr_stacks);
}