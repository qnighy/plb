
use std::cmp;
use std::io;
use std::io::BufRead;
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();

    let mut h : HashMap<String, usize> = HashMap::new();
    let mut max = 0;

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let mut b = false;
        let mut count = 1;
        {
            let x = h.get_mut(&line);
            match x {
                None => {b = true},
                Some(x) => {count = *x + 1; *x = count},
            };
        };
        if b {
            h.insert(line, 1);
        };
        max = cmp::max(max, count);
    }
    println!("{}", h.len());
    println!("{}", max);
}
