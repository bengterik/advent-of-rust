//use std::env;
use std::fs;

fn main() {
    star1();
    star2();
}

fn star1() {
    let contents = fs::read_to_string("calories.in")
    .expect("Should have been able to read file");

    let splitted = contents.split("\n");

    let mut sum = 0;
    let mut temp = 0;

    for s in splitted {
        match s {
            "" => {
                if sum < temp { sum = temp };
                temp = 0;
            },
            _  => temp += s.parse::<i32>().unwrap(),
        }
    }
    println!("{}", sum);
}

fn star2() {

    let contents = fs::read_to_string("calories.in")
        .expect("Should have been able to read file");
    
    let splitted = contents.split("\n");
    
    let mut highest: [i32; 3] = [0, 0, 0]; 
    let mut temp: i32 = 0;

    for s in splitted {
        match s {
            "" => {
                let (i, lowest) = index_of_lowest(&mut highest);
                if lowest < temp { highest[i] = temp };
                temp = 0;
            },
            _  => temp += s.parse::<i32>().unwrap(),
        }
    }
    println!("{}", highest[0]+highest[1]+highest[2]);
}

fn index_of_lowest(arr: &mut [i32; 3]) -> (usize, i32) {
    let mut n = 0;
    let mut lowest = &std::i32::MAX;
    for (i, cal) in arr.iter().enumerate() {
        if cal < lowest {
            n = i;
            lowest = cal;
        }
    }
    return (n, *lowest);

}