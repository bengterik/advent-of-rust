use std::fs;

fn main() {
    star2();
}

fn star2() {
    let ans = String::from(fs::read_to_string("input.in")
                .expect("Should have been able to read file"))
                .split("\n")
                .fold(0, |sum, s| {
                    
                    let s = s
                            .split(",")
                            .map(|l| l.split("-").collect::<Vec<&str>>())
                            .collect::<Vec<Vec<&str>>>();
                    
                    let first = &s[0];
                    let second = &s[1];

                    let f1 = first[0].parse::<i32>().unwrap();
                    let f2 = first[1].parse::<i32>().unwrap();
                    
                    let s1 = second[0].parse::<i32>().unwrap();
                    let s2 = second[1].parse::<i32>().unwrap();
                    
                    if (f1 >= s1 && f1 <= s2) || (f2 >= s1 && f2 <= s2) || (s1 >= f1 && s1 <= f2) || (s2 >= f1 && s2 <= f2) {
                        sum + 1
                    } else {
                        sum
                    }
                });
                    
    println!("{:#?}", ans);
}

fn star1() {
    let ans = String::from(fs::read_to_string("input.in")
                .expect("Should have been able to read file"))
                .split("\n")
                .fold(0, |sum, s| {
                    
                    let s = s
                            .split(",")
                            .map(|l| l.split("-").collect::<Vec<&str>>())
                            .collect::<Vec<Vec<&str>>>();
                    
                    let first = &s[0];
                    let second = &s[1];

                    let f1 = first[0].parse::<i32>().unwrap();
                    let f2 = first[1].parse::<i32>().unwrap();
                    
                    let s1 = second[0].parse::<i32>().unwrap();
                    let s2 = second[1].parse::<i32>().unwrap();

                    if (f1 >= s1 && f2 <= s2) || (s1 >= f1 && s2 <= f2) {
                        sum + 1
                    } else {
                        sum
                    }
                });
                    
    println!("{:#?}", ans);
}