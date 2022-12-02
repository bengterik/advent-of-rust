use std::fs;

fn main() {
    let ans: i32 = fs::read_to_string("in.in")
                .expect("Should have been able to read file")
                .split("\n")
                .map(|v| (v.chars().nth(0).unwrap(), v.chars().nth(2).unwrap()))
                .fold(0, |sum, a| match a { 
                    ('A', 'Y') => sum + 8, // Rock+1 Paper+2
                    ('A', 'X') => sum + 4, // Rock Rock
                    ('A', 'Z') => sum + 3, // Rock Scissor+3
                    ('B', 'Y') => sum + 8, // Paper Paper
                    ('B', 'Z') => sum + 9, // Paper Scissor
                    ('B', 'X') => sum + 1, // Paper Rock
                    ('C', 'Y') => sum + 8, // Scissor Paper
                    ('C', 'X') => sum + 7, // Scissor Rock
                    ('C', 'Z') => sum + 3, // Scissor Scissor
                    _ => 0
                });

    println!("{:?}", ans);
}