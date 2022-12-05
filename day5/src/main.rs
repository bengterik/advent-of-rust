use std::{fs, iter};

fn main() {
    let inp = String::from(fs::read_to_string("in.in")
                .expect("Should have been able to read file"));
    
    let (stacks, moves) = inp.split_once("\n\n").unwrap();

    let nbr_stacks = stacks.split("\n").last().unwrap().chars().filter(|s| s.is_digit(16)).collect::<Vec<char>>().len();

    let stacks = stacks.split('\n')
                        .rev()
                        .collect::<Vec<&str>>()
                        .drain(1..)
                        .map(|p| p.chars().collect::<Vec<char>>())
                        .fold(Vec::new(), |mut v, r | {    
                            for i in (1..nbr_stacks*4-2).step_by(4) {
                                v.push(r[i]); 
                            }
                            v
                        });                            
    
    let mut stack: Vec<Vec<char>> = iter::repeat_with(|| Vec::new()).take(nbr_stacks).collect();    
    
    for (i,&m) in stacks.iter().enumerate() {
        if m != ' ' {
            stack[i%nbr_stacks].push(m);
        }
    }

    let moves = moves.split("\n")
                    .map(|m| m.split(" ").filter_map(|m1| m1.parse::<i32>().ok()).collect::<Vec<i32>>())
                    .map(|m| (m[0], m[1]-1, m[2]-1))
                    .collect::<Vec<(i32,i32,i32)>>();
    
    
    for m in moves {
        move_crates2(&mut stack, m.0, m.1, m.2);
    }


    for s in stack.iter_mut() {
        print!("{}", s.pop().unwrap());
    }
    println!("");
}

fn move_crates1(stack: &mut Vec<Vec<char>>, nbr: i32, from: i32, to: i32) {
    for _ in 0..nbr {
        let popped = stack[from as usize].pop().unwrap();
        stack[to as usize].push(popped);
    }
}

fn move_crates2(stack: &mut Vec<Vec<char>>, nbr: i32, from: i32, to: i32) {
    let mut temp: Vec<char> = Vec::new();
    
    for _ in 0..nbr {
        let popped = stack[from as usize].pop().unwrap();
        temp.push(popped);
    }

    // Push the crates to temp and then to the appropriate stack
    for _ in 0..nbr {
        let popped = temp.pop().unwrap();
        stack[to as usize].push(popped);
    }
}