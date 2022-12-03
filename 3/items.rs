use std::fs;

static LETTERS: [char; 52] = [
    'a', 'b', 'c', 'd', 'e', 
    'f', 'g', 'h', 'i', 'j', 
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 
    'u', 'v', 'w', 'x', 'y', 
    'z', 'A', 'B', 'C', 'D', 
    'E', 'F', 'G', 'H', 'I', 
    'J', 'K', 'L', 'M', 'N', 
    'O', 'P', 'Q', 'R', 'S', 
    'T', 'U', 'V', 'W', 'X', 
    'Y', 'Z',
];

fn main() {
    star1();
}

fn star1() {
    let ans = String::from(fs::read_to_string("input.in")
                .expect("Should have been able to read file"))
                .split("\n")
                .map(|line| line
                            .chars()
                            .collect::<Vec<char>>())
                .fold(0, |sum, cmptmnts| {
                    let mut c = cmptmnts.chunks(cmptmnts.len()/2);
                    let first = c.next().unwrap();
                    let second = c.next().unwrap();
                    
                    let mut m = ' ';
                    
                    for c in first {
                        if second.contains(c) {
                            m = *c;
                        }
                    }

                    sum + LETTERS
                            .iter()
                            .position(|&c| c == m)
                            .unwrap() as i32 + 1
                });

    println!("{:?}", ans);
}