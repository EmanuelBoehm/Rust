mod prims;
use std::io;

fn main() {

    prims::main(read_input());
}

fn read_input() -> Option<usize> {

let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            match input.parse::<usize>() {
                Ok(i) => Some(i),
                Err(_e) => None,
            }
            
        },
        Err(error) => None,
    }
}
