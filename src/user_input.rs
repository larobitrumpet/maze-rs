use std::io::{stdin, stdout, Write};

fn get_input(prompt: &str) -> String {
    let mut line = String::new();
    print!("{}", prompt);
    let _ = stdout().flush();
    stdin().read_line(&mut line).unwrap();
    line.pop();
    line
}

fn get_int<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        let s = get_input(prompt);
        match s.parse() {
            Ok(i) => return i,
            Err(_) => {
                println!("Please enter a positive integer");
                continue;
            },
        }
    }
}

pub fn get_dimentions() -> (usize, usize) {
    println!("Enter the dimentions of the maze:");
    (get_int("  Width: "), get_int("  Height: "))
}

pub fn get_algorithm() -> u32 {
    println!("Select maze generation algorithm:");
    println!("  0: Recursive Backtracking");
    println!("  1: Eller's Algorithm");
    println!("  2: Kruskal's Algorithm");
    println!("  3: Prim's Algorithm");
    println!("  4: Recursive Division");
    println!("  5: Aldous-Broder");
    println!("  6: Wilson's Algorithm");
    println!("  7: Hunt and Kill");
    loop {
        let num: u32 = get_int("Enter maze number: ");
        if num < 8 {
            return num;
        } else {
            println!("Please enter a valid maze number (a number between 0 and 7)");
            continue;
        }
    }
}
