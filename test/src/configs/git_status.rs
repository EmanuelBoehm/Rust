#![allow(dead_code,unused_variables)]
use glob::glob;

struct Table {
    path_list: Vec<std::path::PathBuf>,
    status_list: Vec<String>,
}

impl Table {
    fn new() -> Self {
        Table {
            path_list: Vec::new(),
            status_list: Vec::new(),
        }
    }
}

pub fn main() {
    let mut table = Table::new();
    for entry in glob("/home/emi/**/.git/").expect("Failed to read glob pattern") {
        if let Ok(path) = entry {
            println!("{:?}", &path.display());
            table.path_list.push(path);
        }
    }
    println!("{}", table.path_list.len());
}
