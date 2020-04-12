//runes::Command executes as root!!
use crate::installer::parseprgm::Prgm;
use crate::installer::parseprgm::PrgmType;
use runas;
//process::Command should be used mostly
use std::process;
use dirs;
use std::fs;
use std::path::Path;
use std::env;
mod parseprgm;


pub fn run(path: &Path){
        let sandboxprgm = Prgm::new("testprgm", PrgmType::Aur);

        let prgm_list = parseprgm::parse_prgm(&path);
        match prgm_list {
            Ok(prgm_list) => {
                install_routine(&prgm_list);
            },
            Err(_e) => {
                println!("wroooong");
            }
        }
}

fn install_routine(prgm_list: &Vec<Prgm>){
    let mut pacman_list: Vec<&Prgm> = Vec::new();
    let mut aur_list: Vec<&Prgm> = Vec::new();
    let mut git_list: Vec<&Prgm> = Vec::new();
    let mut suckless_list: Vec<&Prgm> = Vec::new();
    for prgm in prgm_list {
        match prgm.prgm_type {
            PrgmType::Pacman    =>  pacman_list.push(&prgm),
            PrgmType::Aur       =>  aur_list.push(&prgm),
            PrgmType::GitHub    =>  git_list.push(&prgm),
            PrgmType::Suckless  =>  suckless_list.push(&prgm),
        }
    }
    pacman_installer(&pacman_list);
    make_git_dir();
    git_cloner(&aur_list);
    git_cloner(&git_list);
    git_cloner(&suckless_list);
    git_installer();
}

//@todo skip if program is not found
fn pacman_installer(list: &Vec<&Prgm>) {
    let mut prgm_names: Vec<String> = Vec::new();
    for prgm in list {
        prgm_names.push(prgm.name.clone());
    }
    runas::Command::new("pacman")
        .arg("--needed")
        .arg("-S")
        .args(&prgm_names)
        .status()
        .expect("failed to install");
}

fn git_installer() {
    change_dir(vec![".test"]);
    let prgm_dirs = fs::read_dir("/home/emi/.test");

    for prgm_dir in prgm_dirs.unwrap() {
        env::set_current_dir(prgm_dir.unwrap().path()).expect("unable to cd");

        let files = fs::read_dir("./");
        for file in files.unwrap() {
            let path = file.unwrap().path();
            match path.file_name().unwrap().to_str(){
                Some("Makefile") => {
                    runas::Command::new("make")
                        .arg("clean")
                        .arg("install")
                        .status()
                        .expect("failed to install programm");
                    break;
                },
                Some("PKGBUILD") => {
                    process::Command::new("makepkg")
                        .arg("-si")
                        .status()
                        .expect("failed to install programm");
                    break;
                },
                _ => (),
            };
        }
    }
}

fn git_cloner(list: &Vec<&Prgm>){
    change_dir(vec![".test"]);
    for prgm in list {
    process::Command::new("git")
        .arg("clone")
        .arg(&prgm.name)
        .status()
        .expect("failed to install git programm");
    }
}

//seems like it wouldn't create if already exists but needs some more observation
fn make_git_dir() {
    match dirs::home_dir() {
        Some(path) => {
            match fs::create_dir(path.join(Path::new(".test"))) {
                Ok(_x) => println!("created git directory!"),
                Err(_e) => (),
            };
        },
        None => {
            println!("already created user?");
        },
    };
}

fn change_dir(paths: Vec<&str>) {
    match dirs::home_dir() {
        Some(home_path) => {
            let mut path = home_path.clone();
            for p in paths {
                println!("{:?}", p);
                path = path.join(p);
            }
            println!("{:?}", path);
            env::set_current_dir(path).expect("unable to cd");
        },
        None => println!("wasn't able to change working directory to git dir"),
    };
}
