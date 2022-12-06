use std::{fs};
use itertools::Itertools;

fn main() {
    msg_start(4); // star 1 
    msg_start(14); // star 2
}

fn msg_start(seq_length: usize) {
    let inp = String::from(fs::read_to_string("in.in")
                .expect("Should have been able to read file"))
                .chars()
                .collect::<Vec<char>>()
                .windows(seq_length)
                .try_fold(0, |index: usize, w| {
                    if w.iter().all_unique() {
                        Err(index+seq_length) 
                    } else {
                        Ok(index+1)
                    }
                });

    let ans = match inp {
                    Err(i) => i,
                    Ok(_) => 0,
                };
    println!("{:#?}", ans);
}