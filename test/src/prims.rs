use std::io::{stdin, stdout, Write};

//read user input and returns it as String
fn read() -> usize {
    let mut input = String::new();
    stdout().flush().expect("failed to flush");
    stdin().read_line(&mut input).expect("failed to read");
    return input.trim().parse().unwrap();
}

fn get_sieve(len: usize) -> Vec<bool> {
    let mut table = vec![true; len];
    for num in 2..table.len() {
        for multiple in (num * num..table.len()).step_by(num) {
            table[multiple] = false;
        }
    }
    table
}

fn pretty_print(list: Vec<bool>) {
    let mut output: Vec<usize> = Vec::new();
    for num in 2..list.len() {
        if list[num] {
            output.push(num);
        }
    }
    println!("{:?}", output)
}

pub fn main(int: Option<usize>) {
    println!("what should be the prim bound?");
    //fills empty prim list assuming that every number is prim at start
    //with users input as upper bound
    let list: Vec<bool> = match int {
            Some(int)   => get_sieve(int),
            None        => get_sieve(read()),
        };
    //unvalue all numbers which aren't prim
    pretty_print(list);
}
