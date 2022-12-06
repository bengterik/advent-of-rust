use std::{fs};
use itertools::Itertools;

fn main() {
    star1();
    star2();
}

fn star2() {
    let inp = String::from(fs::read_to_string("in.in")
                .expect("Should have been able to read file"))
                .chars()
                .collect::<Vec<char>>()
                .windows(14)
                .try_fold(0, |index: i32, w| {
                    if w.iter().all_unique() {
                        Err(index+14) 
                    } else {
                        Ok(index+1)
                    }
                });

    let ans = match inp {
                    Err(i) => i,
                    Ok(_) => -1,
                };
    println!("{:#?}", ans);
}

fn star1() {
    let inp = String::from(fs::read_to_string("in.in")
                .expect("Should have been able to read file"))
                .chars()
                .collect::<Vec<char>>()
                .windows(4)
                .try_fold(0, |index: i32, w| {
                    if w.iter().all_unique() {
                        Err(index+4) 
                    } else {
                        Ok(index+1)
                    }
                });

    let ans = match inp {
                    Err(i) => i,
                    Ok(_) => -1,
                };
    println!("{:#?}", ans);
}