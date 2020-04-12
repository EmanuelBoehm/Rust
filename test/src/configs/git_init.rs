use git2::Repository;
use std::path::Path;
use dirs::home_dir;

struct GitConfigs {
    vec: Vec<i32>,
}

pub fn init(path: Option<&Path>) {
    let alternative = dirs::home_dir().unwrap().join("configs");
    let pathh = path.unwrap_or(alternative.as_path());
    let repo = match Repository::init(pathh) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to init: {}", e),
    };
}

//fill
fn fill() {
    return
}
