use std::path::Path;
use csv::Reader;
use std::error::Error;

/* get the csv file, which stores all programs I want to install
 * and returns a HashMap with <name,installer>
 * currently installer has "" for pacman and "git" for git installs
 */

pub enum PrgmType {
    Pacman,
    Aur,
    GitHub,
    Suckless
}

pub struct Prgm {
    pub name: String,
    path: Option<Box<Path>>,
    pub prgm_type: PrgmType,
    download_link: Option<String>,
}



pub fn parse_prgm(path: &std::path::Path) -> Result<Vec<Prgm>, Box<dyn Error>> {
    let mut programs: Vec<Prgm> = Vec::new();
    let mut reader = Reader::from_path(&path)?;
    for program_entry in reader.records() {
        let record = program_entry.expect("a csv record");
        let program_type = match &record[0] {
            "git" | "Git" | "g" | "G" => PrgmType::GitHub,
            "Suckless" | "suckless" | "sl" => PrgmType::Suckless,
            "Aur" | "AUR" | "aur" => PrgmType::Aur,
            _ => PrgmType::Pacman,
        };
        programs.push(Prgm::new(&record[1], program_type))
    }

    return Ok(programs);
}

fn path_to_name(path: &str) -> String {
    let tokens = path.split("/").collect::<Vec<&str>>();
    match tokens.last().unwrap().clone(){
        ".git" => return tokens.get(tokens.len()-2).unwrap().to_string(),
        _ => return tokens.last().unwrap().to_string(),
    };
}

impl Prgm {
    pub fn new(name: &str, prgm_type: PrgmType) -> Prgm {
        match prgm_type {
            PrgmType::Pacman => {
                return Prgm {
                    name: name.to_string(),
                    path: None,
                    prgm_type: prgm_type,
                    download_link: None,
                };
            },
            PrgmType::Aur | PrgmType::GitHub | PrgmType::Suckless => {
                return Prgm {
                    name: path_to_name(name),
                    path: Option::from(Box::from(Path::new("/home/emi/.test"))),
                    prgm_type: prgm_type,
                    download_link:Option::from(name.to_string()),
                }
            }
        }
    }


}
